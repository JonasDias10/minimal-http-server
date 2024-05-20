use std::fs::File;
use std::io::{self, Read, Write};

const STORAGE_DIRECTORY: &str = "/storage/";

/**
 * Get the root path of the server
 * @return the root path
 */
fn get_root_path() -> String {
    std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

/**
 * Save a file to the server storage
 * @param {string} filename
 * @param {u8[]} content
 */
pub fn save_file(filename: &str, content: &[u8]) -> io::Result<()> {
    let path = format!("{}{}{}", get_root_path(), STORAGE_DIRECTORY, filename);
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(content).expect("Unable to write data");
    Ok(())
}

/**
 * Get a file from the server storage
 * @param filename
 * @return the file
 */
pub fn get_file(filename: &str) -> io::Result<Vec<u8>> {
    let path = format!("{}{}{}", get_root_path(), STORAGE_DIRECTORY, filename);

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
