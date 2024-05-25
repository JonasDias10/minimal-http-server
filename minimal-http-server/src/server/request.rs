use std::io::{Read, Result};
use std::net::TcpStream;

const BUFFER_SIZE: usize = 1024;
const HEADER_END_SEQUENCE: &[u8; 4] = b"\r\n\r\n";

pub struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl Request {
    pub fn parse(stream: &mut TcpStream) -> Result<Self> {
        let mut buffer = Vec::new();
        let mut temp_buffer = [0; BUFFER_SIZE];

        let bytes_read = stream.read(&mut temp_buffer)?;
        buffer.extend_from_slice(&temp_buffer[..bytes_read]);

        let (headers_part, body_part) = split_buffer(&buffer)?;

        let headers_string = String::from_utf8_lossy(headers_part);
        let headers_lines: Vec<&str> = headers_string.lines().collect();

        let (method, path, version) = parse_request_first_line(headers_lines[0]);
        let headers = parse_headers(&headers_lines[1..]);

        let content_length = get_content_length(&headers);

        let mut body = Vec::from(body_part);
        if body.len() < content_length {
            body.resize(content_length, 0);
            stream.read_exact(&mut body[body_part.len()..])?;
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
 * Get content length from headers.
 * If not found, return 0
 * @param headers
 * @return content length
 */
fn get_content_length(headers: &[(String, String)]) -> usize {
    headers
        .iter()
        .find(|(key, _value)| key.eq_ignore_ascii_case("Content-Length"))
        .and_then(|(_key, value)| value.parse::<usize>().ok())
        .unwrap_or(0)
}

/**
 * Parse headers from headers lines.
 * @param headers_lines
 * @return headers
*/
fn parse_headers(headers_lines: &[&str]) -> Vec<(String, String)> {
    let mut headers: Vec<(String, String)> = Vec::new();
    for line in headers_lines.iter().skip(1) {
        if line.trim().is_empty() {
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
 * Parse request first line.
 * @param request_first_line
 * @return (method, path, version)
 */
fn parse_request_first_line(request_first_line: &str) -> (String, String, String) {
    let parts: Vec<&str> = request_first_line.split_whitespace().collect();
    let method = parts.get(0).unwrap_or(&"").to_string();
    let path = parts.get(1).unwrap_or(&"").to_string();
    let version = parts.get(2).unwrap_or(&"").to_string();
    (method, path, version)
}

/**
 * Get final position of headers.
 * @param buffer
 * @return final position
 */
fn get_final_position_of_headers(buffer: &[u8]) -> Option<usize> {
    buffer
        .windows(HEADER_END_SEQUENCE.len())
        .position(|window| window == HEADER_END_SEQUENCE)
        .map(|pos| pos + HEADER_END_SEQUENCE.len())
}

/**
 * Split buffer into headers part and body part.
 * @param buffer
 * @return (headers part, body part)
 */
fn split_buffer(buffer: &[u8]) -> Result<(&[u8], &[u8])> {
    let final_position_headers = get_final_position_of_headers(&buffer);

    match final_position_headers {
        Some(final_position_headers) => Ok(buffer.split_at(final_position_headers)),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid request",
        )),
    }
}
