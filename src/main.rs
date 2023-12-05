mod util_lib;
pub use crate::util_lib::util;
pub use crate::util_lib::config;
pub use crate::util_lib::read_doc;

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{prelude::*, BufReader};
use std::ops::Add;
use clap::Parser;
use util_lib::config::Cli;


fn main() {
    println!("Starting Server...");

    let cli: Cli = config::Cli::parse();
    println!("Using server config for start up : \n{:?}", cli);

    let port = String::from("127.0.0.1:").add(&cli.port.to_string());

    let listner = TcpListener::bind(port).unwrap();
    println!("************ Server started ************");
    for stream in listner.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let cli: Cli = config::Cli::parse();
    let buf_reader = BufReader::new(&mut stream);
    let http_request:String = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .map(String::from)
        .take_while(|line| !line.is_empty())
        .collect();
    
    let request_parts = http_request.split_whitespace().collect::<Vec<_>>();
    println!("Recived Request :  {:?}",  request_parts[1].to_string());
    let requested_path: String = request_parts[1].to_string();

    let fpath = util::get_file_path(requested_path, cli.path );

    stream.write_all(read_doc::read_doc_to_resp(fpath).as_bytes()).unwrap();
}
