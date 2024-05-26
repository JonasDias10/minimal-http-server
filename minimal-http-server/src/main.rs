mod server;

use server::client;
use std::io::Result;
use std::net::TcpListener;

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .unwrap_or_else(|error| panic!("Failed to bind to port 8080: {}", error));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match client::handle_client(stream) {
                Ok(()) => (),
                Err(error) => println!("Error when handling client: {}", error),
            },
            Err(error) => println!("Error when creating stream: {}", error),
        }
    }

    Ok(())
}
