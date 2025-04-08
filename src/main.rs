#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Error as E, Read, Write, BufReader};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").expect("Failed to bind to the addr");
    //
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
    let mut reader = BufReader::new(&stream);
    let mut buf: [u8; 1024] = [0; 1024];

    let bytes_read = reader.read(&mut buf).expect("Failed to read data from the input buffer");
    if bytes_read == 0{
        return;
    }
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    let _ = stream.write_all(response.as_bytes()).expect("Failed to write to the buffer");
}
