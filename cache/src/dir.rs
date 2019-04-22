use std::env::temp_dir;
use std::fs::create_dir_all;
use std::io;
use std::path::PathBuf;

#[cfg(test)]
const CACHE_DIR_NAME: &str = "flora-cache-test";
#[cfg(not(test))]
const CACHE_DIR_NAME: &str = "flora-cache";

pub fn get_cache_dir() -> io::Result<PathBuf> {
    let mut path = temp_dir();
    path.push(CACHE_DIR_NAME);
    create_dir_all(&path)?;
    Ok(path)
}
