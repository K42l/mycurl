use crate::response;
use std::io::{self, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use openssl::ssl::{SslConnector, SslMethod};

use response::handle_response;

pub fn handle_connection(
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

pub fn openssl_connection(
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