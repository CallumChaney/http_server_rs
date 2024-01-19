use response::{ResponseBuilder, StatusCode};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

mod response;

const HOST: &str = "127.0.0.1";
const PORT: &str = "7878";

const GET_INDEX: &str = "GET / HTTP/1.1";
fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("{}:{}", HOST, PORT))
        .expect(&format!("Could not connect to server at {}:{}", HOST, PORT));
    println!("Listening at {}:{}", HOST, PORT);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)?;
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap()?;

    let (status_code, file_name) = if request_line == GET_INDEX {
        (StatusCode::Ok, "index.html")
    } else {
        (StatusCode::NotFound, "404.html")
    };

    let response = ResponseBuilder::new()
        .status_code(status_code)
        .content(file_name)
        .build();

    stream.write_all(response.generate_response_string().as_bytes())?;
    Ok(())
}
