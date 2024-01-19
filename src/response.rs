use std::fs;

const STATUS_LINE_BASE: &str = "HTTP/1.1";

pub struct Response {
    status_line: String,
    content: String,
}

impl Response {
    fn new() -> Self {
        Self {
            status_line: "".to_owned(),
            content: "".to_owned(),
        }
    }

    pub fn generate_response_string(&self) -> String {
        let length = self.content.len();
        let response_string = format!(
            "{}\r\nContent-Length: {length}\r\n\r\n{}",
            self.status_line, self.content
        );
        response_string
    }
}
pub enum StatusCode {
    Ok,
    NotFound,
}

pub struct ResponseBuilder {
    response: Response,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {
            response: Response::new(),
        }
    }
    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.response.status_line = match status_code {
            StatusCode::Ok => STATUS_LINE_BASE.to_owned() + " 200 OK",
            StatusCode::NotFound => STATUS_LINE_BASE.to_owned() + " 404 NOT FOUND",
        };
        self
    }
    pub fn content(mut self, path: &str) -> Self {
        self.response.content = fs::read_to_string(path).expect("Expected file path");
        self
    }
    pub fn build(self) -> Response {
        self.response
    }
}
