mod server;

use server::client;
use std::io::Result;
use std::net::TcpListener;

fn main() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => client::handle_client(stream).unwrap(),
            Err(error) => println!("Error: {}", error),
        }
    }

    Ok(())
}
