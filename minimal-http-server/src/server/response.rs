use std::io::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    Created,
    NotFound,
    BadRequest,
    InternalServerError,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Ok => "200 OK",
            Status::Created => "201 Created",
            Status::NotFound => "404 Not Found",
            Status::BadRequest => "400 Bad Request",
            Status::InternalServerError => "500 Internal Server Error",
        }
    }
}

pub struct Response {
    pub version: String,
    pub status: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Response {
    fn create_headers(content_type: &str) -> Vec<(String, String)> {
        Vec::new()
    }

    fn create_body(body: &[u8]) -> Vec<u8> {
        Vec::new()
    }

    pub fn create_response(status: Status, body: &[u8], content_type: &str) -> Result<Self> {
        Ok(Response {
            version: String::from("HTTP/1.1"),
            status: status.as_str().to_string(),
            headers: Response::create_headers(content_type),
            body: Response::create_body(body),
        })
    }
}
