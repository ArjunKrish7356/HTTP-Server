#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::{collections::HashMap, io::{BufReader, Read, Write}, path::Path, time::Duration};
use rayon::{vec, ThreadPoolBuilder};
use std::{fs::File, env};


const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
const BAD_REQUEST_RESPONSE: &str = "HTTP/1.1 400 Bad Request\r\n\r\n";
const BIND_ADDRESS: &str = "127.0.0.1:4221";
const RESOURCE_CREATED: &str = "HTTP/1.1 201 Created\r\n\r\n";

fn extract_headers(request: &str) -> HashMap<String,String> {
    let mut headers = HashMap::new();
    let mut splitted_request = request.split("\r\n");

    if let Some(status) = splitted_request.next() {
        let splitted_status: Vec<&str> = status.splitn(3," ").collect();
        if splitted_status.len() == 3 {
            headers.insert("Type".to_string(), splitted_status[0].to_string());
            headers.insert("Route".to_string(), splitted_status[1].to_string());
            headers.insert("Version".to_string(), splitted_status[2].to_string());
        } else {
            eprintln!("Malformed status line: {}", status);
        }
    }
    

    for split in splitted_request {
        if let Some((key, value)) = split.split_once(':') {
            headers.insert(
                key.trim().to_string(),
                value.trim().to_string(), // Trim whitespace
            );
       } else if !split.is_empty() { // Ignore empty lines but log others
           eprintln!("Malformed header encountered: {}", split);
       }
    }
    headers
}

fn main() -> Result<(),std::io::Error> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind(BIND_ADDRESS)?;
    let pool = match ThreadPoolBuilder::new().num_threads(8).build() {
        Ok(answer) => answer,
        Err(e) => panic!("Failed to build thread pool: {:?}", e)
    };
    
    for stream in listener.incoming() {
         match stream {
             Ok(stream) => {
                pool.spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling connection: {}", e);
                    }
                });
                },
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(),std::io::Error>{
    loop{
        let buf_reader = BufReader::new(&stream);
        stream
            .set_read_timeout(Some(Duration::new(0, 100000000)))
            .expect("Timeout handled");

        if let Ok(response) = handle_request(buf_reader) {
            if let Err(e) = stream.write_all(&response) {
            eprintln!("Failed to send response: {}", e);
            }
        } else {
            eprintln!("Error processing request");
        }
    }
}

fn handle_request(mut reader: BufReader<&TcpStream>) -> Result<Vec<u8>,std::io::Error>{
    let mut buf: [u8; 1024] = [0; 1024];

    let bytes_read = match reader.read(&mut buf) {
        Ok(0) => {
            println!("Client Disconnectd");
            return Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Disconnected"));
        },
        Ok(n) => n,
        Err(e) => {
            eprintln!("Failed to read from stream: {}", e);
            return Err(e);
        }
    };

    let request = String::from_utf8_lossy(&buf[..bytes_read]);
    let headers = extract_headers(&request);
    println!("{:#?}",headers);

    let response = match (headers.get("Type").map(|s| s.as_str()), headers.get("Route").map(|s| s.as_str())) {
        (Some("GET"), Some("/")) => OK_RESPONSE.to_string(),
        (Some("GET"), Some(route)) if route.starts_with("/echo/") => {
            if let Some(param) = route.strip_prefix("/echo/") {
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    param.len(),
                    param
                )
            } else {
                BAD_REQUEST_RESPONSE.to_string()
            }
        },
        (Some("GET"), Some("/user-agent")) => {
            if let Some(user_agent) = headers.get("User-Agent") {
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    user_agent.len(),
                    user_agent
                )
            } else {
                BAD_REQUEST_RESPONSE.to_string()
            }
        },
        (Some("GET"), Some(route)) if route.starts_with("/files/") => {
            if let Some(file_name) = route.strip_prefix("/files/") {
                let env_args: Vec<String> = env::args().collect();
                let mut dir = env_args[2].clone();
                dir.push_str(file_name);
                match std::fs::read(&dir) {
                    Ok(content) => {
                        format!(
                            "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                            content.len(),
                            String::from_utf8_lossy(&content)
                        )
                    },
                    Err(_) => NOT_FOUND_RESPONSE.to_string()
                }
            } else {
                NOT_FOUND_RESPONSE.to_string()
            }
        },
        (Some("POST"), Some(route)) if route.starts_with("/files/") => {
            let env_args: Vec<String> = env::args().collect();
            let dir_name = &env_args[2];
            let filename = route.strip_prefix("/files/").expect("Error while stripping file");
            let file_path = Path::new(dir_name).join(filename);
            
            // Properly split the request into headers and body using \r\n\r\n separator
            let parts: Vec<&str> = request.split("\r\n\r\n").collect();
            if parts.len() > 1 {
                let body = parts[1];
                
                match File::create(&file_path) {
                    Ok(mut file) => {
                        if let Err(e) = file.write_all(body.as_bytes()) {
                            eprintln!("Failed to write to file {}: {}", file_path.display(), e);
                            NOT_FOUND_RESPONSE.to_string()
                        } else {
                            RESOURCE_CREATED.to_string()
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to create file {}: {}", file_path.display(), e);
                        NOT_FOUND_RESPONSE.to_string()
                    }
                }
            } else {
                eprintln!("Request body not found in the POST request");
                NOT_FOUND_RESPONSE.to_string()
            }
        },
        _ => NOT_FOUND_RESPONSE.to_string(), // default response for any other method/route
    };
    println!("{}",response);

    Ok(response.as_bytes())
}
