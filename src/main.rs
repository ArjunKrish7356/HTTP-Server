#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind to the addr");
    let addr = listener.local_addr().expect("could't read the ip address");
    for stream in listener.incoming() {
         match stream {
             Ok(_stream) => {
                 handle_request(_stream);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}

fn handle_request(mut stream: TcpStream){
    let addr = stream.local_addr().expect("could't read the ip address");
    let mut reader = BufReader::new(&stream);
    let mut buf: [u8; 1024] = [0; 1024];

    let bytes_read = reader.read(&mut buf).expect("Failed to read data from the input buffer");
    if bytes_read == 0{
        return;
    };

    let request = String::from_utf8_lossy(&buf[..bytes_read]);
    let split_request: Vec<&str> = request.split("/").collect();
    let path = split_request[1];

    let response = match path {
        " HTTP" =>  "HTTP/1.1 200 OK\r\n\r\n",
        _ => "HTTP/1.1 404 Not Found\r\n\r\n"
    };

    stream.write_all(response.as_bytes()).expect("Error occured while writing to buffer");
}
