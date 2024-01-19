use crate::response::{ResponseBuilder, StatusCode};
use std::{
    collections::HashMap,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub enum Method {
    GET,
}
#[derive(Debug)]
pub struct Route {
    method: Method,
    path: String,
    file_path: Option<String>,
}

impl Route {
    pub fn new_get(path: &str, file_path: &str) -> Self {
        Self {
            method: Method::GET,
            path: path.to_owned(),
            file_path: Some(file_path.to_owned()),
        }
    }

    fn get_request_line(&self) -> String {
        match self.method {
            Method::GET => return format!("GET {} HTTP/1.1", self.path),
        }
    }
}

pub struct Server {
    listener: TcpListener,
    routes: Vec<Route>,
}

impl Server {
    pub fn new<T: Into<String> + std::fmt::Display>(host: T, port: T) -> Self {
        let listener = TcpListener::bind(format!("{host}:{port}"))
            .expect(&format!("Could not connect to server at {host}:{port}"));

        Self {
            listener,
            routes: Vec::new(),
        }
    }

    pub fn listen(&mut self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();
            handle_connection(stream, &self.routes).unwrap();
        }
    }
    pub fn regester_route(&mut self, route: Route) -> Result<(), ()> {
        if !self
            .routes
            .iter()
            .take_while(|r| r.path == route.path)
            .collect::<Vec<&Route>>()
            .is_empty()
        {
            return Err(());
        }

        println!("{:?}", &route.get_request_line());
        self.routes.push(route);
        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream, routes: &Vec<Route>) -> std::io::Result<()> {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap()?;

    let (status_code, file_name) = if let Some(route) = routes
        .iter()
        .filter(|route| request_line == route.get_request_line())
        .collect::<Vec<&Route>>()
        .first()
    {
        println!("{:?}", route);
        match route.method {
            Method::GET => (StatusCode::NotFound, route.file_path.clone().unwrap()),
        }
    } else {
        (StatusCode::Ok, "404.html".to_owned())
    };

    let response = ResponseBuilder::new()
        .status_code(status_code)
        .content(&file_name)
        .build();

    stream.write_all(response.generate_response_string().as_bytes())?;
    Ok(())
}
