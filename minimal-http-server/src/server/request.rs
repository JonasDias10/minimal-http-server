use std::io::{Read, Result};
use std::net::TcpStream;

const BUFFER_SIZE: usize = 1024;

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    // fn new() -> Self {
    //     Request {
    //         method: String::new(),
    //         path: String::new(),
    //         version: String::new(),
    //         headers: Vec::new(),
    //         body: Vec::new(),
    //     }
    // }

    /**
     * Parse the request from the stream
     * @param stream: TcpStream
     * @return Request
     */
    pub fn parse(stream: &mut TcpStream) -> Result<Self> {
        let mut buffer = [0; BUFFER_SIZE];

        stream.read(&mut buffer)?;

        let request_as_string = String::from_utf8_lossy(&buffer);
        let lines: Vec<&str> = request_as_string.lines().collect();

        // Take the first line and parse it to the method, path and version
        let first_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
        let method = first_line_parts.get(0).unwrap_or(&"").to_string();
        let path = first_line_parts.get(1).unwrap_or(&"").to_string();
        let version = first_line_parts.get(2).unwrap_or(&"").to_string();

        // Parse the headers
        let mut headers: Vec<(String, String)> = Vec::new();
        let mut data_start_index = None;
        for (index, line) in lines.iter().enumerate().skip(1) {
            if line.trim().is_empty() {
                data_start_index = Some(index + 1);
                break;
            }
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() == 2 {
                headers.push((parts[0].to_string(), parts[1].to_string()));
            }
        }

        let content_length = get_content_length(&headers);
        let mut body = Vec::new();

        // Parse the body if it exists
        if content_length > 0 {
            let mut body_bytes = Vec::new();

            // Read the body from the lines after the headers
            if let Some(start_index) = data_start_index {
                for line in lines.iter().skip(start_index) {
                    body_bytes.extend_from_slice(line.as_bytes());
                    body_bytes.push(b'\n');
                }

                body_bytes.truncate(content_length);
                body.append(&mut body_bytes);
            }

            // Read the rest of the body from the stream
            let total_read = body.len();
            body.resize(content_length, 0);
            if total_read < content_length {
                stream.read_exact(&mut body[total_read..])?;
            }
        }

        Ok(Request {
            method,
            path,
            version,
            headers,
            body,
        })
    }
}

fn get_content_length(headers: &Vec<(String, String)>) -> usize {
    headers
        .iter()
        .find(|&(ref key, ref _value)| key.eq_ignore_ascii_case("Content-Length"))
        .map(|(_key, value)| value.parse::<usize>().unwrap_or(0))
        .unwrap_or(0)
}
