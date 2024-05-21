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

        let mut data_start_index = None;
        let headers = get_headers(&lines, &mut data_start_index);

        let content_length = get_content_length(&headers);
        let mut body = Vec::new();

        // Parse the body if it exists
        if content_length > 0 {
            // Read the body from the lines after the headers
            let mut body_bytes = get_remaining_request_data(&lines, data_start_index);
            body_bytes.truncate(content_length);
            body.append(&mut body_bytes);

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

/**
 * Get the content length from the headers
 * @param headers
 * @return content length
 */
fn get_content_length(headers: &Vec<(String, String)>) -> usize {
    headers
        .iter()
        .find(|&(ref key, ref _value)| key.eq_ignore_ascii_case("Content-Length"))
        .map(|(_key, value)| value.parse::<usize>().unwrap_or(0))
        .unwrap_or(0)
}

/**
 * Get the headers from the request
 * @param request
 * @return headers
 */
fn get_headers(request: &Vec<&str>, data_start_index: &mut Option<usize>) -> Vec<(String, String)> {
    let mut headers: Vec<(String, String)> = Vec::new();
    for (index, line) in request.iter().enumerate().skip(1) {
        if line.trim().is_empty() {
            *data_start_index = Some(index + 1);
            break;
        }
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            headers.push((parts[0].to_string(), parts[1].to_string()));
        }
    }
    headers
}

/**
 * Get the remaining request data after the headers
 * @param request
 * @param start_index
 * @return partial body
 */
fn get_remaining_request_data(request: &Vec<&str>, start_index: Option<usize>) -> Vec<u8> {
    let mut body_bytes = Vec::new();

    if let Some(index) = start_index {
        for line in request.iter().skip(index) {
            body_bytes.extend_from_slice(line.as_bytes());
            body_bytes.push(b'\n');
        }
    }

    body_bytes
}
