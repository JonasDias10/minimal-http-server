use super::{request, storage};
use std::io::Result;

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
    NotFound,
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Ok => "200 OK",
            Status::NotFound => "404 Not Found",
        }
    }
}

pub struct Response {
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn create_response(request: &request::Request) -> Result<Response> {
        let mut body = Vec::new();
        let mut status = Status::Ok;

        match request.method.as_str() {
            "GET" => match storage::get_file(&request.path) {
                Ok(buffer) => {
                    body = buffer;
                }
                Err(exception) => {
                    println!("Error: {}", exception);

                    let content = response_body_when_has_error(status.as_str());

                    body = content.into_bytes();

                    status = Status::NotFound;
                }
            },
            method => {
                println!("Unknown request method: {}", method);
            }
        }

        let content_type = get_content_type(&request.path);
        let content_length = body.len();

        let headers = create_headers(&content_type, content_length, status.as_str());

        Ok(Response { headers, body })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut response = Vec::new();

        for (key, value) in &self.headers {
            response.extend_from_slice(key.as_bytes());
            if !value.is_empty() {
                response.extend_from_slice(b": ");
                response.extend_from_slice(value.as_bytes());
            }
            response.extend_from_slice(b"\r\n");
        }

        response.extend_from_slice(b"\r\n");

        response.extend_from_slice(&self.body);

        response
    }
}

fn create_headers(content_type: &str, length: usize, status: &str) -> Vec<(String, String)> {
    let mut headers = Vec::new();

    let fist_line = format!("HTTP/1.1 {}", status);
    headers.push((fist_line, "".to_string()));
    headers.push(("Content-Type".to_string(), content_type.to_string()));
    headers.push(("Content-Length".to_string(), length.to_string()));

    headers
}

fn get_content_type(path: &str) -> String {
    let file_type = path.split('.').last();

    let content_type = match file_type {
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        _default => "text/plain",
    };

    content_type.to_string()
}

fn response_body_when_has_error(error_message: &str) -> String {
    format!(
        r#"
            <!DOCTYPE html>
                <html lang="pt-BR">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Minimal HTTP Server</title>
                </head>
                <body>
                    <div id="container">
                        <h1>Minimal HTTP Server</h1>
                        <h2>{}</h2>
                    </div>
                </body>
            </html>
        "#,
        error_message
    )
}
