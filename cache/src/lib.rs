mod dir;

use bincode::deserialize_from;
use dir::get_cache_dir;
use lazy_static::lazy_static;
use lru_disk_cache::LruDiskCache;
use serde::de::DeserializeOwned;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

const MAX_BYTES: u64 = 64 * 1024 * 1024;

fn create_cache() -> Option<LruDiskCache> {
    match get_cache_dir() {
        Err(err) => {
            eprintln!("Failed to access cache directory - {}", err);
            None
        }
        Ok(cache_dir) => match LruDiskCache::new(cache_dir, MAX_BYTES) {
            Err(err) => {
                eprintln!("Failed to access disk cache - {:?}", err);
                None
            }
            Ok(lru_disk_cache) => Some(lru_disk_cache),
        },
    }
}
lazy_static! {
    static ref CACHE: Option<Mutex<LruDiskCache>> = create_cache().map(Mutex::new);
}

pub fn get<K, V>(key: K) -> Option<V>
where
    K: Hash,
    V: DeserializeOwned,
{
    let mut hasher = DefaultHasher::new();
    hasher.write(env!("CARGO_PKG_VERSION").as_bytes());
    key.hash(&mut hasher);
    let hash = format!("{:x}", hasher.finish());

    let result: Result<Option<V>, String> = match CACHE.as_ref() {
        None => Err("Cache unavailable".to_string()),
        Some(mutex) => match mutex.lock() {
            Err(err) => Err(format!("Cache lock failed - {}", err)),
            Ok(mut cache) => match cache.get(hash) {
                Err(lru_disk_cache::Error::FileNotInCache) => Ok(None),
                Err(err) => Err(format!("Cache get failed - {}", err)),
                Ok(read) => match deserialize_from(read) {
                    Err(err) => Err(format!("Cache deserialization failed - {}", err)),
                    Ok(value) => Ok(Some(value)),
                },
            },
        },
    };

    match result {
        Ok(option) => option,
        Err(err) => {
            eprintln!("{}", err);
            None
        }
    }
}

// TODO extract cache access
// TODO set

#[cfg(test)]
mod tests {
    // use super::*;
    // TODO
}
