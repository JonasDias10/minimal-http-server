use super::request::Request;
use super::response::{self};
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
        "GET" => {
            let response = match response::Response::create_response(&request) {
                Ok(response) => response,
                Err(err) => {
                    println!("Failed to create response: {}", err);
                    return Ok(());
                }
            };

            let response = response.as_bytes();
            stream.write(&response).expect("Failed to write to stream");
        }
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
