mod cli;
mod util;
use core::str;
use std::io::{self, BufWriter, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::fs::File;
use std::ops::Index;
use std::path::PathBuf;
use std::sync::Arc;
use openssl::ssl::{SslConnector, SslMethod};
use rustls::{client, ClientConfig, ProtocolVersion, RootCertStore};
use webpki_roots;
use httparse::Response;

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
    let location = matches.get_flag("location");

    let (protocol, hostname, pathname, socket_addr) = parse_url(url);
    let socket_test = socket_addr.to_socket_addrs();

    match socket_test{
        Ok(socket) => {
            if socket.as_ref().len() > 0{
                for s in socket {
                    println!("Connecting: {}", &socket_addr);
                    let request = populate_request(protocol, &hostname.clone(), &pathname, data, method, headers.clone());
                    let buffer: Result<Vec<u8>, std::io::Error>;
                    if socket_addr.contains("443"){
                        buffer = openssl_connection(&s.to_string(), hostname.clone(), &request, verbose);      
                    } else {
                        buffer = http_connection(&s.to_string(), &request, verbose);
                    }
                    match buffer{
                        Ok(buffer) => {
                            if buffer.is_empty(){
                                continue;
                            }else {
                                if location {
                                    let mut response_headers = [httparse::EMPTY_HEADER; 64];
                                    let mut response = Response::new(&mut response_headers);
                                    let _ = httparse::Response::parse(& mut response, &buffer).unwrap();
                                    match response.code.unwrap() {
                                        301 | 302 => {
                                            for header in response.headers.into_iter() {
                                                if header.name == "Location"{
                                                    let new_path = str::from_utf8(header.value).unwrap();
                                                    let request2 = populate_request(protocol, &hostname.clone(), new_path, data, method, headers.clone());
                                                    let buffer2: Result<Vec<u8>, std::io::Error>;
                                                    if socket_addr.contains("443"){
                                                        buffer2 = openssl_connection(&s.to_string(), hostname.clone(), &request2, verbose);      
                                                    } else {
                                                        buffer2 = http_connection(&s.to_string(), &request2, verbose);
                                                    }
                                                    match buffer2 {
                                                        Ok(buffer) => {
                                                            match write_response(verbose, include, &buffer, &output){
                                                                Ok(response) => response,
                                                                Err(err) => return Err(err),
                                                            }
                                                        }
                                                        Err(err) => return Err(err)
                                                    }
                                                }                   
                                            }
                                        }
                                        _ => match write_response(verbose, include, &buffer, &output){
                                            Ok(response) => response,
                                            Err(err) => return Err(err),
                                        }
                                    }
                                }  else {
                                    match write_response(verbose, include, &buffer, &output){
                                        Ok(response) => response,
                                        Err(err) => return Err(err),
                                    };
                                }                           
                                
                                break;
                            }
                           
                        }
                        Err(err) => return Err(err)
                    }
                    
                }
                return Ok(())
            }
            else {
                println!("No address found");
                Ok(())
            }            
        }
        Err(err) => return Err(err)
    }
}

/*
fn tls_connection(
    socket_addr: &str, 
    hostname: String, 
    request: &str,  
    verbose: bool
) -> Result<Vec<u8>, std::io::Error>{
    //let root_store = rustls::RootCertStore::empty();
    let root_store = rustls::RootCertStore::from_iter(
        webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .cloned(),
    );
    let config = rustls::ClientConfig::builder()
                                .with_root_certificates(root_store)
                                .with_no_client_auth();

    let rc_config = Arc::new(config);
    let client_connection = rustls::ClientConnection::new(rc_config, hostname.try_into().unwrap());
    match client_connection{
        Ok (mut client) => {
            if verbose {
                let lines = request.lines();
                println!("Request Headers:");
                for line in lines {
                    println!("> {line}");
                }
            }             
            let mut sock = TcpStream::connect(socket_addr).unwrap();
            let mut tls = rustls::Stream::new(&mut client, &mut sock);

            tls.write_all(request.as_bytes()).unwrap();
            
            
            let ciphersuite = tls.conn
                                .negotiated_cipher_suite()
                                .unwrap();
            if verbose {
                println!("Current ciphersuite: {:?}",ciphersuite.suite())
            }

            let mut buffer = Vec::new();

            tls.read_to_end(&mut buffer).unwrap();

            Ok(buffer)
        }
        Err(err) =>{
            let std_err = std::io::Error::new(io::ErrorKind::Other, err.to_string());
            return Err(std_err)
        } 
    }
}
*/

fn openssl_connection(
    socket_addr: &str, 
    hostname: String, 
    request: &str,  
    verbose: bool
) -> Result<Vec<u8>, std::io::Error>{
    let connector = SslConnector::builder(SslMethod::tls()).unwrap().build();

    let stream = TcpStream::connect(socket_addr).unwrap();
    let stream = connector.connect(&hostname, stream);

    match stream{
        Ok(mut ssl_stream) => {
            if verbose {
                let lines = request.lines();
                println!("Request Headers:");
                for line in lines {
                    println!("> {line}");
                }
            }  

            ssl_stream.write_all(request.as_bytes()).unwrap();
            let mut buffer = Vec::new();

            ssl_stream.read_to_end(&mut buffer).unwrap();
            
            Ok(buffer)
        }
        Err(err) =>{
            let std_err = std::io::Error::new(io::ErrorKind::Other, err.to_string());
            return Err(std_err)
        }
    }

}

fn http_connection( 
    socket_addr: &str, 
    request: &str, 
    verbose: bool
) -> Result<Vec<u8>, std::io::Error>{
    let tcp_socket = TcpStream::connect(&socket_addr);
    match tcp_socket {
        Ok(mut stream) =>{            
            if verbose {
                let lines = request.lines();
                println!("Request Headers:");
                for line in lines {
                    println!("> {line}");
                }
            }
            stream.write_all(request.as_bytes())
                .expect("Faile to write to stream");

            let mut buffer = Vec::new();
            stream.read_to_end(&mut buffer)
                .expect("Failed to read response from host");

            Ok(buffer)
        }
        Err(err) => return Err(err)
    }
}

fn write_response(
    verbose: bool, 
    include: bool, 
    buffer: &[u8], 
    output: &Option<&PathBuf>
) -> Result<(), std::io::Error>{
    let resp = String::from_utf8_lossy(&buffer);
    let (resp_header, resp_data) = (&resp).split_once("\r\n\r\n").unwrap();

    let mut handle = BufWriter::new(match &output{
        Some(ref path) => Box::new(File::create(path).unwrap()) as Box<dyn Write>,
        None => Box::new(io::stdout()) as Box<dyn Write>
    });

    if verbose || include{
        let lines = resp_header.split("\r\n");
        println!("Response Headers:");
        for line in lines{
            if verbose {
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