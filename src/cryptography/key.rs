use serde::{Deserialize, Serialize};

use super::digest::{sha256_hash, sha512_hash};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyErr {
    ParseFailure(String),
    SerializeFailure(String),
}

impl std::fmt::Display for KeyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(self).expect("should never fail to deserialize")
        )
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Algorithms {
    #[serde(rename = "rsa-sha256")]
    RsaSha256,
    /// is actually Ed25519-SHA512
    #[serde(rename = "hs2019")]
    Hs2019,
}

impl Algorithms {
    /// hash a body with the respective hashing algorithm and outputs `SHA-__=hash`
    pub fn hash(&self, body: &[u8]) -> String {
        match self {
            Algorithms::RsaSha256 => format!("SHA-256={}", sha256_hash(body)),
            Algorithms::Hs2019 => format!("SHA-512={}", sha512_hash(body)),
        }
    }
}

impl std::fmt::Display for Algorithms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Algorithms::RsaSha256 => write!(f, "rsa-sha256"),
            Algorithms::Hs2019 => write!(f, "hs2019"),
        }
    }
}
impl TryFrom<&str> for Algorithms {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "rsa-sha256" => Ok(Self::RsaSha256),
            "hs2019" => Ok(Self::Hs2019),
            _ => Err(()),
        }
    }
}

pub trait Key: Sized {
    /// Serialize from PEM
    fn from_pem(pem: &str) -> Result<Self, KeyErr>;
    /// Serialize self to PEM.
    /// if a public key this will be the public pem
    fn to_pem(&self) -> Result<String, KeyErr>;
    fn algorithm(&self) -> Algorithms;
}

pub trait PrivateKey: Key + Clone {
    /// sign the provided content with this key
    fn sign(&mut self, content: &[u8]) -> String;
    // fn from_pem(pem: &str, algorithm: crate::cryptography::key::KeyType) -> Result<Self, ParseErr>;
    fn generate(algorithm: Algorithms) -> Self;
    // fn private_key_pem(&self) -> String;
    fn public_key_pem(&self) -> Result<String, KeyErr>;
}

pub trait PublicKey: Key + Clone {
    /// verify that the provided content was signed with this key
    fn verify(&self, plain_content: &[u8], signature: &[u8]) -> bool;
    // fn from_pem(pem: &str, algorithm: KeyType) -> Result<Self, ParseErr>;
}
