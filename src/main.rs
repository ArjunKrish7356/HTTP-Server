#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::{collections::HashMap, io::{BufReader, Read, Write}};

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
const BAD_REQUEST_RESPONSE: &str = "HTTP/1.1 400 Bad Request\r\n\r\n";
const BIND_ADDRESS: &str = "127.0.0.1:4221";

fn extract_headers(request: &str) -> HashMap<String,String> {
    let mut headers = HashMap::new();
    let mut splitted_request = request.split("\r\n");

    if let Some(status) = splitted_request.next() {
        let splitted_status: Vec<&str> = status.split(" ").collect();
        headers.insert("Type".to_string(), splitted_status[0].to_string());
        headers.insert("Route".to_string(), splitted_status[1].to_string());
        headers.insert("Version".to_string(), splitted_status[2].to_string());

    }

    for split in splitted_request {
        let header_splitted: Vec<&str> = split.split(":").collect();
        if header_splitted.len() >= 2 {
            headers.insert(
                header_splitted[0].trim().to_string(),
                header_splitted[1].trim().to_string(),
            );
        }
    }
    headers
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind(BIND_ADDRESS).expect("Failed to bind to the addr");
    
    for stream in listener.incoming() {
         match stream {
             Ok(stream) => {
                if let Err(e) = handle_request(stream) { // Renamed _stream to stream
                    eprintln!("Error handling connection: {}", e);
                }
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}

fn handle_request(mut stream: TcpStream) -> Result<(),std::io::Error>{
    let mut reader = BufReader::new(&stream);
    let mut buf: [u8; 1024] = [0; 1024];

    let bytes_read = match reader.read(&mut buf){
        Ok(0) => {
            println!("Client Disconnectd");
            return Ok(());
        },
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
           return Err(e);
        }
    };

    let request = String::from_utf8_lossy(&buf[..bytes_read]);
    let header = extract_headers(&request);
    let mut response = String::new();

    if let (Some(type_value), Some(route_value)) = (header.get("Type"),header.get("Route")) {
        if type_value == "GET" && route_value == "/" {
            response = OK_RESPONSE.to_string();
        }
        else if type_value == "GET" && route_value.starts_with("/echo/"){
            let splitted: Vec<&str> = route_value.split("/").collect();
            let param = splitted[2];
            let length = param.len();
            response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", length, param);
        }
        else if type_value == "GET" && route_value.starts_with("/user-agent") {
            if let Some(user_agent) = header.get("User-Agent") {
                response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", user_agent.len(), user_agent);
            }
        }
        else {
            response = NOT_FOUND_RESPONSE.to_string();
        }
    }
    println!("{}",response);

    if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write response: {}", e);
    }

    Ok(())
}
