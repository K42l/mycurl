mod cli;
mod util;
use core::str;
use std::io::{self, BufWriter, Error, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::fs::File;
//use std::ops::Index;
use std::path::{self, PathBuf};
//use std::sync::Arc;
use openssl::ssl::{SslConnector, SslMethod};
//use rustls::{client, ClientConfig, ProtocolVersion, RootCertStore};
//use webpki_roots;
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

    let ( protocol, hostname, pathname,  socket_addr) = parse_url(url);
    let request = populate_request(protocol, &hostname.clone(), &pathname, data, method, headers.clone());
    
    let buffer_result = handle_connection(  &request, 
                                                                    &hostname, 
                                                                    socket_addr, 
                                                                    verbose, 
                                                                    location, 
                                                                    protocol, 
                                                                    pathname, 
                                                                    data, 
                                                                    method, 
                                                                    &headers
                                                                );

    match buffer_result {
        Ok(buffer) => {
            match write_response(verbose, include, &buffer, &output){
                Ok(response) => return Ok(response),
                Err(err) => return Err(err),
            }
        }
        Err(err) => return Err(err)
    }

}

fn handle_connection(
    request: &String, 
    hostname: &String, 
    socket: String,
    verbose: bool,
    location: bool,
    protocol: &str,
    pathname: &str,
    data: Option<&String>,
    method: Option<&String>,
    headers: &Vec<&str>
) -> Result<Vec<u8>, std::io::Error>{
    let socket_result = socket.to_socket_addrs();

    match socket_result{
        Ok(sockets) => {
            if sockets.as_ref().len() > 0{
                for s in sockets {
                    
                    let buffer_result: Result<Vec<u8>, std::io::Error>;
                    if s.to_string().contains("443"){
                        buffer_result = openssl_connection(&s.to_string(), hostname.clone(), &request, verbose);      
                    } else {
                        buffer_result = http_connection(&s.to_string(), &request, verbose);
                    }
                    
                    match buffer_result{
                        Ok(buffer) => {
                            if buffer.is_empty(){
                                continue;
                            }else {
                                return handle_response(
                                    &hostname, 
                                    s.to_string(), 
                                    verbose, 
                                    location, 
                                    protocol, 
                                    pathname, 
                                    data, 
                                    method, 
                                    &headers,
                                    &buffer
                                );
                            }
                        }
                        Err(err) => return Err(err)
                    }
                }
                return Err(std::io::Error::new(io::ErrorKind::Other, "Empty response"))
            } else {
                let std_err = std::io::Error::new(io::ErrorKind::Other, "No address found");
                return Err(std_err)
            } 
        }
        Err(err) => return Err(err)
    }
}

fn handle_response(
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
                println!("Connecting with OpenSSL");
                println!("Socket addr: {:?}", socket_addr);
                println!("Hostname: {:?}", hostname);
                println!("\r");
                let lines = request.lines();
                println!("Request Headers:");
                for line in lines {
                    println!("> {line}"); 
                }
                println!("\r")
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
                println!("Http Connection starrted");
                println!("Socket addr: {:?}", socket_addr);
                println!("\r");
                let lines = request.lines();
                println!("Request Headers:");
                for line in lines {
                    println!("> {line}");
                }
                println!("\r")
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