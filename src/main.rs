mod cli;
mod util;
use std::io::{self, BufWriter, Read, Write};
use std::net::TcpStream;
use std::fs::File;

use cli::get_arguments;
use util::{parse_url, populate_request};

fn main() -> Result<(), std::io::Error>{
    let matches = get_arguments();

    let verbose = matches.get_flag("verbose");
    let url = matches.get_one::<String>("url").unwrap();
    let data = matches.get_one::<String>("data");
    let method = matches.get_one::<String>("method");
    let headers:Vec<&str> = matches.get_many::<String>("header")
                                            .unwrap_or_default()
                                            .map(|s| s.as_str())
                                            .collect();
    let output = matches.get_one::<std::path::PathBuf>("file");
    let include = matches.get_flag("include");

    let (protocol, hostname, pathname, socket_addr) = parse_url(url);

    let buffer_str = populate_request(protocol, hostname, &pathname, data, method, headers);

    let tcp_socket = TcpStream::connect(socket_addr);
    match tcp_socket {
        Ok(mut stream) =>{
            if verbose {
                let lines = buffer_str.lines();
                println!("Request Headers:");
                for line in lines {
                    println!("> {line}");
                }
            }

            stream
                .write_all(buffer_str.as_bytes())
                .expect("Faile to write to stream");

            let mut buffer = Vec::new();
            stream
                .read_to_end(&mut buffer)
                .expect("Failed to read response from host");

            let resp = String::from_utf8_lossy(&buffer);
            let (resp_header, resp_data) = (&resp).split_once("\r\n\r\n").unwrap();

            let mut handle = BufWriter::new(match &output{
                Some(ref path) => Box::new(File::create(path).unwrap()) as Box<dyn Write>,
                None => Box::new(io::stdout()) as Box<dyn Write>
            });

            if verbose || include{
                let lines = resp_header.split("\r\n");
                for line in lines{
                    if verbose && output.is_some() {
                        println!("< {line}")
                    }
                    if include {
                        writeln!(handle, "< {line}")?;
                    }
                }
            }
            writeln!(handle, "{resp_data}")?;
            Ok(())
        }
        Err(err) =>{
            return Err(err)
        }
    }
}