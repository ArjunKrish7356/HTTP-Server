#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Error as E, Read, Write};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
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
    let mut buf: [u8; 1024] = [0; 1024];

    let _ = stream.read(&mut buf);
    if buf.len() == 0{
        return;
    }
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    let _ = stream.write_all(response.as_bytes());
}
