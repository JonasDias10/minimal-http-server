use super::request::Request;
use super::storage;
use std::io::{Result, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) -> Result<()> {
    let request = match Request::parse(&mut stream) {
        Ok(req) => req,
        Err(err) => {
            println!("Failed to parse request: {}", err);

            return Ok(());
        }
    };

    match request.method.as_str() {
        "GET" => match storage::get_file(&request.path) {
            Ok(buffer) => {
                let content_type = match request.path.split('.').last() {
                    Some(extension) => match extension {
                        "html" => "text/html",
                        "css" => "text/css",
                        "js" => "application/javascript",
                        "json" => "application/json",
                        "svg" => "image/svg+xml",
                        "png" => "image/png",
                        "jpg" | "jpeg" => "image/jpeg",
                        _ => "text/plain",
                    },
                    None => "text/plain",
                };

                let headers = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
                    content_type,
                    buffer.len()
                );

                let body = String::from_utf8_lossy(&buffer[..]);

                let response = format!("{}{}", headers, body);

                stream
                    .write(response.as_bytes())
                    .expect("Failed to write to stream");

                stream.flush().expect("Failed to flush stream");

                println!("File downloaded successfully!");
            }
            Err(exception) => {
                println!("Error: {}", exception);

                let content = format!("Error: {}", exception).into_bytes();
                stream.write(&content).expect("Failed to write to stream");
            }
        },
        "POST" => match storage::save_file(&request.path, &request.body) {
            Ok(()) => {
                println!("File saved successfully!");
            }
            Err(exception) => {
                println!("Error: {}", exception);
            }
        },
        method => {
            println!("Unknown request method: {}", method);

            stream
                .write(b"Unknown request method")
                .expect("Failed to write to stream");

            stream.flush().expect("Failed to flush stream");
        }
    }

    Ok(())
}
