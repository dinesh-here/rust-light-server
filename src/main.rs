use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{prelude::*, BufReader};

fn main() {
    println!("Starting Server...");
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server started");
    for stream in listner.incoming() {
        let stream = stream.unwrap();
        println!("Connected..");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut req_str = String::new();
    buf_reader.read_to_string(&mut req_str).expect("Error on read");

    let request_parts = req_str.split_whitespace().collect::<Vec<_>>();
    let requested_path = request_parts[1];
        
    // const file_path: String = req_path[1];
    println!("{}",req_str);
    println!("Path {:#?}",requested_path);

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
}
