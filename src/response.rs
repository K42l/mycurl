use crate::util;
use crate::connection;
use httparse::Response;
use core::str;
use std::io::{self, BufWriter,Write};
use std::path::PathBuf;
use std::fs::File;

use util::{parse_url, populate_request};
use connection::handle_connection;

pub fn handle_response(
    hostname: &String, 
    socket: String,
    verbose: bool,
    location: bool,
    protocol: &str,
    pathname: &str,
    data: Option<&String>,
    method: Option<&String>,
    headers: &Vec<&str>,
    buffer: &Vec<u8>
) -> Result<Vec<u8>, std::io::Error>{
    if location {
        let mut response_headers = [httparse::EMPTY_HEADER; 64];
        let mut response = Response::new(&mut response_headers);
        let _ = httparse::Response::parse(& mut response, &buffer).unwrap();
        match response.code.unwrap() {
            301 | 302 | 307 => {
                if verbose {
                    println!("Response code: {:?}", response.code.unwrap())
                }
                for header in response.headers.into_iter() {
                    if header.name == "Location"{
                        if verbose {
                            println!("Following location: {:?}", str::from_utf8(header.value).unwrap());
                            println!("\r");
                        }
                        let new_path = str::from_utf8(header.value).unwrap();
                        if new_path != pathname{
                            if new_path.contains("http"){
                                let (new_protocol, _, pathname, new_socket) = parse_url(new_path);
                                let new_request = populate_request(new_protocol, &hostname, pathname, data, method, headers.clone());
                                return handle_connection(&new_request, 
                                    hostname, 
                                    new_socket, 
                                    verbose, 
                                    location, 
                                    protocol, 
                                    pathname,
                                    data,
                                    method,
                                    headers);
                            } else {
                                let new_request = populate_request(protocol, &hostname, new_path, data, method, headers.clone());
                                return handle_connection(&new_request, 
                                    hostname, 
                                    socket.clone(), 
                                    verbose, 
                                    location, 
                                    protocol, 
                                    pathname,
                                    data,
                                    method,
                                    headers);
                            }                          
                        } else {
                            return Ok(buffer.to_vec())
                        }
                    }                   
                }
                return Err(std::io::Error::new(io::ErrorKind::Other, "Empty header"))
            }
            
            _ => return Ok(buffer.to_vec())
            
        }
    }  else {
        return Ok(buffer.to_vec())
    }  
}

pub fn write_response(
    verbose: bool, 
    include: bool, 
    buffer: &[u8], 
    output: &Option<&PathBuf>
) -> Result<(), std::io::Error>{
    let resp = String::from_utf8_lossy(buffer);
    let (resp_header, resp_data) = (resp).split_once("\r\n\r\n").unwrap();

    let mut handle = BufWriter::new(match &output{
        Some(ref path) => Box::new(File::create(path).unwrap()) as Box<dyn Write>,
        None => Box::new(io::stdout()) as Box<dyn Write>
    });

    if verbose || include{
        let lines = resp_header.split("\r\n");
        if verbose {
            println!("Response Header count: {:?}", &lines.clone().count());
            println!("Response body length: {:?}", resp_data.len());
            println!("\r")
        }
        println!("Response Headers:");
        for line in lines{
            if verbose {
                println!("< {line}")
            }
            if include {
                writeln!(handle, "< {line}")?;
            }
        }
        writeln!(handle, "\r")?;
    }
    writeln!(handle, "{resp_data}")?;
    Ok(())
}