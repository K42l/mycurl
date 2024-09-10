use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static!{
    static ref PROTOCOL_STRING: HashMap<&'static str,&'static str> = {
        let mut map = HashMap::new();
        map.insert("http", "HTTP/1.1");
        map.insert("https", "HTTP/1.1");
        map
    };
}

const DEFAULT_PORT: &str = "80";
const DEFAULT_PORT_S: &str = "443";

pub fn parse_url(url: &str) -> (&str, String, &str, String){
    let (protocol, rest) = url.split_once("://").unwrap_or(("https",url));
    let (temp_hostname, pathname) = rest.split_once("/").unwrap_or((rest, ""));
    let (hostname, port) = if temp_hostname.contains(":"){
        temp_hostname.split_once(":").expect("Invalid hostname")
    } else {
        (temp_hostname, if protocol == "https"{DEFAULT_PORT_S} else {DEFAULT_PORT})
    };
    let socket_addr = format!("{hostname}:{port}");
    let protocol_str = PROTOCOL_STRING.get(protocol).expect("invalid protocol");

    (protocol_str, hostname.to_string(), pathname, socket_addr)
}

pub fn populate_request(
    protocol: &str,
    host: &str,
    path: &str,
    data: Option<&String>,
    method: Option<&String>,
    headers: Vec<&str>
) -> String{
    let method = method.unwrap();
    let mut req = String::new();

    req += &format!("{method} /{path} {protocol}\r\n");
    req += &format!("Host: {host}\r\n");
    req += "Connection: close\r\n";
    req += "User-Agent: mcurl/0.1.0\r\n";
    req += "Accept: */*\r\n";

    if method == "POST" || method == "PUT"{
        if headers.len() > 0{
            for v in headers {
                req += v;
            }
            req += "\r\n"
        } else {
            req += "Content-Type: application/json\r\n"
        }
        match data {
            Some(d) => {
                let data_len = d.as_bytes().len();
                req += &format!("Content-Length: {data_len}\r\n\r\n");
                req += d;
                req += "\r\n";
            }
            _ => ()
        }
    }
    req += "\r\n";
    req
}