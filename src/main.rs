mod cli;
mod util;
mod response;
mod connection;
use core::str;
//use std::ops::Index;
//use std::sync::Arc;
//use rustls::{client, ClientConfig, ProtocolVersion, RootCertStore};
//use webpki_roots;

use cli::get_arguments;
use util::{parse_url, populate_request};
use response::write_response;
use connection::handle_connection;

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