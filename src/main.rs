#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, Read, Write};
// Removed the unnecessary anyhow::Ok import.

// use anyhow::Ok;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind to the addr");
    
    for stream in listener.incoming() {
         match stream {
             Ok(_stream) => {
                let _ = handle_request(_stream);
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
            return Ok(());
        }
    };

    let request = String::from_utf8_lossy(&buf[..bytes_read]);
    let split_request: Vec<&str> = request.trim().split_whitespace().collect();
    
    if split_request.len() >= 3 {
        let _method = split_request[0];
        let path = split_request[1];
        let _http_version = split_request[2];

        let response = match path {
            "/" =>  "HTTP/1.1 200 OK\r\n\r\n",
            _ => "HTTP/1.1 404 Not Found\r\n\r\n"
        };

        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write response: {}", e);
        }
    }  else {
        eprintln!("Malformed request line: {}", request.trim());
        // Consider sending a 400 Bad Request response
        let response = "HTTP/1.1 400 Bad Request\r\n\r\n";
         if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write bad request response: {}", e);
        }
    }

    Ok(())
}
