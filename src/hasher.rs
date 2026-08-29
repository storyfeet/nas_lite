use std::pin::{Pin, pin};
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

pub async fn hash_file_by_path(path: &str) -> Result<String, tokio::io::Error> {
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
            let res = hash_file_by_path("./data/simple_file_for_tests.txt");
            assert_eq!(64, res.await.unwrap().len());
        });
    }
}
