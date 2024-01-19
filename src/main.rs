use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const HOST: &str = "127.0.0.1";
const PORT: &str = "7878";

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))
        .expect(&format!("Could not connect to server at {}:{}", HOST, PORT));
    println!("Listening at {}:{}", HOST, PORT);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}
