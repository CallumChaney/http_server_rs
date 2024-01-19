use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const HOST: &str = "127.0.0.1";
const PORT: &str = "7878";

const GET_INDEX: &str = "GET / HTTP/1.1";

const STATUS_200_OK: &str = "HTTP/1.1 200 OK";
const STATUS_400_NOTFOUND: &str = "HTTP/1.1 404 NOT FOUND";

enum RequestLine {
    GET(String),
}

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

    let (status_line, file_name) = if request_line == GET_INDEX {
        (STATUS_200_OK, "index.html")
    } else {
        (STATUS_400_NOTFOUND, "404.html")
    };

    let contents = fs::read_to_string(file_name)?;
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes())?;
    Ok(())
}
