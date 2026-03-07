use std::fs;
use std::path::PathBuf;


// Verify if the path exists
pub fn is_exist(path: &PathBuf) -> Option<&PathBuf> {
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

// Read the file else Error
pub fn read_file(path: &PathBuf) -> Option<String> {
    match fs::read_to_string(&path) {
        Ok(content) => Some(content),
        Err(err) => {
            eprintln!("Asto CLI Error - Error while read {:?}. {}", path, err);
            std::process::exit(1);
        }
    }
} 

// Verify if the file received is from the extension Asto
pub fn is_asto_type(path: &PathBuf) -> bool {
    match path.extension() {
        Some(ext) => ext == "asto",
        None => false
    }
}