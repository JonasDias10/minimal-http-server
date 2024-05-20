mod server;

use server::request::Request;
use server::storage;
use std::io::{Result, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_client(stream).unwrap(),
            Err(error) => println!("Error: {}", error),
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let request = match Request::parse(&mut stream) {
        Ok(req) => req,
        Err(err) => {
            println!("Failed to parse request: {}", err);

            return Ok(());
        }
    };

    match request.method.as_str() {
        "GET" => {
            let filename = normalize_filename(&request.path);

            match storage::get_file(&filename) {
                Ok(buffer) => {
                    let content =
                        format!("{}{}", String::from_utf8_lossy(&buffer[..]), "\n").into_bytes();
                    stream.write(&content).expect("Failed to write to stream");
                }
                Err(exception) => {
                    println!("Error: {}", exception);

                    let content = format!("Error: {}", exception).into_bytes();
                    stream.write(&content).expect("Failed to write to stream");
                }
            }
        }
        "POST" => {
            let filename = normalize_filename(&request.path);

            match storage::save_file(&filename, &request.body) {
                Ok(()) => {
                    println!("File saved successfully!");
                }
                Err(exception) => {
                    println!("Error: {}", exception);
                }
            }
        }
        method => {
            println!("Unknown request method: {}", method);
        }
    }

    Ok(())
}

fn normalize_filename(filename: &str) -> String {
    filename.split('/').last().unwrap().trim().to_string()
}
