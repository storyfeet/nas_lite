use std::{
    path::Path,
    pin::{Pin, pin},
};
use tokio::io::{AsyncRead, AsyncReadExt};

pub async fn hash<R: AsyncRead>(mut reader: Pin<&mut R>) -> Result<String, tokio::io::Error> {
    let mut buf: [u8; 10000] = [0; 10000];

    let mut b_hasher = blake3::Hasher::new();

    loop {
        match reader.as_mut().read(&mut buf).await {
            Ok(0) => {
                return Ok(b_hasher.finalize().to_string());
            }
            Ok(num_bytes) => {
                b_hasher.update(&buf[0..num_bytes]);
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

pub fn hash_bytes(data: &[u8]) -> String {
    let mut b_hasher = blake3::Hasher::new();
    b_hasher.update(data);
    // Note if updating , update all finalizes to match
    b_hasher.finalize().to_string()
}

pub fn hash_hashes<'a, R: AsRef<str>>(hashes: &'a [R]) -> String {
    // Sort the hashes to make sure the result is consistent for any order
    let mut hvec: Vec<&'a str> = hashes.iter().map(|s| s.as_ref()).collect();

    hvec.sort();

    let mut b_hasher = blake3::Hasher::new();
    for hash in hvec {
        b_hasher.update(hash.as_bytes());
    }

    b_hasher.finalize().to_string()
}

pub async fn hash_file_by_path<P: AsRef<Path>>(path: P) -> Result<String, tokio::io::Error> {
    let reader = tokio::fs::OpenOptions::new().read(true).open(path).await?;

    hash(pin!(reader)).await
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn can_hash_file() {
        let rt = tokio::runtime::Runtime::new().expect("Could not start runtime");

        rt.block_on(async {
            let res = hash_file_by_path::<&str>("./data/simple_file_for_tests.txt");
            assert_eq!(64, res.await.unwrap().len());
        });
    }

    #[test]
    pub fn hash_hashes_sorts_correctly() {
        let a = hash_hashes(&["123", "456", "hello"]);
        let b = hash_hashes(&["hello", "456", "123"]);

        assert_eq!(a, b);
    }
}
