#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read, Write};

const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
const BAD_REQUEST_RESPONSE: &str = "HTTP/1.1 400 Bad Request\r\n\r\n";
const BIND_ADDRESS: &str = "127.0.0.1:4221";

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
    let split_request: Vec<&str> = request.trim().split_whitespace().collect();
    
    if split_request.len() >= 3 {
        let _method = split_request[0];
        let path = split_request[1];
        let _http_version = split_request[2];
        println!("{} , {} , {}",_method, path, _http_version);

        let mut response = match path {
            "/" =>  OK_RESPONSE.to_string(),
            _ => NOT_FOUND_RESPONSE.to_string()
        };

        if path.starts_with("/echo/") {
            let prefix = path.strip_prefix("/echo/").unwrap_or("");
            response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",prefix.len(),prefix);
            println!("{}",response);
        }

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write response: {}", e);
        }
        } else {
        eprintln!("Malformed request line: {}", request.trim());
        // Consider sending a 400 Bad Request response
        let response = BAD_REQUEST_RESPONSE;
        stream.write_all(response.as_bytes())?;       
    }

    Ok(())
}
