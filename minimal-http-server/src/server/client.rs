use super::request::Request;
use super::response::{self, Status};
use super::storage;
use std::io::{Result, Write};
use std::net::TcpStream;

const GET_METHOD: &str = "GET";
const POST_METHOD: &str = "POST";

/**
 * Handles a single client connection.
 * @param stream - The client connection
 */
pub fn handle_client(mut stream: TcpStream) -> Result<()> {
    let request = match Request::parse(&mut stream) {
        Ok(req) => req,
        Err(error) => return Err(error),
    };

    match request.method.as_str() {
        GET_METHOD => match handle_get_request(&request, stream) {
            Ok(()) => {
                println!("Request {} handled successfully", request.method);
            }
            Err(error) => return Err(error),
        },
        POST_METHOD => match handle_post_request(&request) {
            Ok(()) => {
                println!("Request {} handled successfully", request.method);
            }
            Err(error) => return Err(error),
        },
        method => {
            println!("Unknown request method: {}", method);

            match handle_unknown_request(stream) {
                Ok(()) => {}
                Err(error) => return Err(error),
            }
        }
    }

    Ok(())
}

/**
 * Handles a GET request.
 * @param request - The request
 * @param stream - The client connection
 */
fn handle_get_request(request: &Request, mut stream: TcpStream) -> Result<()> {
    let response = match response::Response::create_response(&request) {
        Ok(response) => response,
        Err(error) => return Err(error),
    };

    let response = response.as_bytes();
    match stream.write(&response) {
        Ok(bytes_written) => {
            println!("Wrote {} bytes to stream", bytes_written);
        }
        Err(error) => return Err(error),
    }

    match stream.flush() {
        Ok(()) => {
            println!("Stream flushed successfully");
        }
        Err(error) => return Err(error),
    }

    Ok(())
}

/**
 * Handles a POST request.
 * @param request - The request
 * @param stream - The client connection
 */
fn handle_post_request(request: &Request) -> Result<()> {
    match storage::save_file(&request.path, &request.body) {
        Ok(()) => {
            println!("File saved successfully!");
        }
        Err(error) => return Err(error),
    };
    Ok(())
}

/**
 * Handles an unknown request.
 * @param request - The request
 * @param stream - The client connection
 */
fn handle_unknown_request(mut stream: TcpStream) -> Result<()> {
    let body = body_when_unknown_request(Status::NotAllowed.as_str());
    let content_type = "text/html";
    let length = body.len();
    let headers = response::create_headers(content_type, length, Status::NotAllowed.as_str());

    let response = response::Response::new(headers, body).as_bytes();
    match stream.write(&response) {
        Ok(bytes_written) => {
            println!("Wrote {} bytes to stream", bytes_written);
        }
        Err(error) => return Err(error),
    }

    match stream.flush() {
        Ok(()) => {
            println!("Stream flushed successfully");
        }
        Err(error) => return Err(error),
    }

    Ok(())
}

/**
 * Creates a body when an unknown request is received.
 * @param error_message - The error message
 * @return The body
 */
fn body_when_unknown_request(error_message: &str) -> Vec<u8> {
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
    .into_bytes()
}
