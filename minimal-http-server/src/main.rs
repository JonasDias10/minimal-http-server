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
        "GET" => match storage::get_file(&request.path) {
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
        }
    }

    Ok(())
}
