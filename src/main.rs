mod util_lib;
pub use crate::util_lib::util;
pub use crate::util_lib::config;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{prelude::*, BufReader};
use std::fs;
use std::ops::Add;
// use std::env;

use clap::Parser;
use util_lib::config::Cli;


fn main() {
    println!("Starting Server...");

    let cli: Cli = config::Cli::parse();
    println!("{:?}", cli);

    let port = String::from("127.0.0.1:").add(&cli.port.to_string());

    let listner = TcpListener::bind(port).unwrap();
    println!("Server started");
    for stream in listner.incoming() {
        let stream = stream.unwrap();
        println!("Connected..");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let cli: Cli = config::Cli::parse();
    println!("{:?}", cli);
    let buf_reader = BufReader::new(&mut stream);
    let http_request:String = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .map(String::from)
        .take_while(|line| !line.is_empty())
        .collect();

    let request_parts = http_request.split_whitespace().collect::<Vec<_>>();
    let requested_path: String = request_parts[1].to_string();

    let fpath = util::get_file_path(requested_path, cli.path );
        
    println!("{:#?}",fpath);
    let status_line = "HTTP/1.1 200 OK";

    let contents = fs::read_to_string(fpath).unwrap_or(String::from("read_to_string"));


    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
