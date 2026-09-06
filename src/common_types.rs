/*!
 * This module provides classes to be used by both client server as a means of communication.
 */
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use serde::{Deserialize, Serialize, de::Visitor};
use structopt::StructOpt;

#[derive(Debug, StructOpt, Deserialize, Clone, Serialize)]
#[structopt()]
pub struct UserPassword {
    pub name: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]

pub struct SessionData {
    pub token: String,
    pub token_pass: String,
    pub expires: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Token {
    pub token: String,
    pub token_pass: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUpload {
    token: Token,
    file_name: String,
    file_hash: String,
    byte_start: u64,
    byte_end: u64,
    data: Chunk,
}

#[derive(Debug, Clone)]
pub struct Chunk(Vec<u8>);

impl Serialize for Chunk {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let b64 = URL_SAFE.encode(&self.0);
        serializer.serialize_str(&b64)
    }
}

impl<'de> Deserialize<'de> for Chunk {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_str(ByteVisitor {})
            .map(|d| Chunk(d))
    }
}

struct ByteVisitor {}

impl<'de> Visitor<'de> for ByteVisitor {
    type Value = Vec<u8>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(URL_SAFE
            .decode(v)
            .map_err(|_| serde::de::Error::custom("Could not decode bytes as base64"))?)
    }
}

#[cfg(test)]
pub mod test_common_types {
    use super::{Chunk, FileUpload};

    #[test]
    pub fn test_serialize_can_make_u64() {
        let upload = FileUpload {
            token: super::Token {
                token: "hello".to_string(),
                token_pass: "hello_pass".to_string(),
            },
            file_hash: "abc".to_string(),
            file_name: "fname".to_string(),
            byte_start: 0,
            byte_end: 100,
            data: Chunk(b"make a pie".to_vec()),
        };

        let s = serde_json::to_string(&upload).unwrap();

        let d: FileUpload = serde_json::from_str(&s).unwrap();

        assert_eq!(d.data.0, upload.data.0);
    }
}
