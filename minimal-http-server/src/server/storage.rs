use std::fs::File;
use std::io::{self, Read, Write};

const STORAGE_DIRECTORY: &str = "/storage/";
const ALLOWED_FILE_EXTENSIONS: [&str; 7] = ["jpg", "jpeg", "png", "svg", "html", "js", "css"];

/**
 * Get the root path of the server.
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
 * Save a file to the server storage.
 * @param filename
 * @param content
 */
pub fn save_file(filename: &str, content: &[u8]) -> io::Result<()> {
    let filename_normalized = normalize_filename(filename);

    if !is_allowed_extension(&filename_normalized) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File extension not allowed",
        ));
    }

    let path = format!(
        "{}{}{}",
        get_root_path(),
        STORAGE_DIRECTORY,
        filename_normalized
    );

    let mut file = File::create(path)?;
    file.write_all(content)?;

    Ok(())
}

/**
 * Get a file from the server storage.
 * @param filename
 * @return the file
 */
pub fn get_file(filename: &str) -> io::Result<Vec<u8>> {
    let filename_normalized = normalize_filename(filename);

    if !is_allowed_extension(&filename_normalized) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File extension not allowed",
        ));
    }

    let path = format!(
        "{}{}{}",
        get_root_path(),
        STORAGE_DIRECTORY,
        filename_normalized
    );

    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/**
 * Normalize the filename.
 * @param filename
 * @return the normalized filename
 */
fn normalize_filename(filename: &str) -> String {
    filename.split('/').last().unwrap().trim().to_string()
}

/**
 * Check if the filename is allowed.
 * @param filename
 * @return true if the filename is allowed, false otherwise
 */
fn is_allowed_extension(filename: &str) -> bool {
    ALLOWED_FILE_EXTENSIONS
        .iter()
        .any(|extension| filename.ends_with(extension))
}
