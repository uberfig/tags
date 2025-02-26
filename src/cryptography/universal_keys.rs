//! for rsa mastodon uses pkcs1-v1.5 with sha256 as seen:
//! 
//! https://docs.joinmastodon.org/spec/security/#http-sign


use super::{digest::{sha256_hash, sha512_hash}, key::{Algorithms, Key, KeyErr, PrivateKey, PublicKey}};
use ed25519_dalek::VerifyingKey;
use rand::rngs::OsRng;
use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey}, signature::{Keypair, SignerMut}
};
use sha2::Sha256;

const LINE_ENDING: pkcs8::LineEnding = pkcs8::LineEnding::LF;

#[derive(Debug, Clone)]
pub enum UniversalPrivate {
    RSA(rsa::pkcs1v15::SigningKey<Sha256>),
    Ed35519(ed25519_dalek::SigningKey),
}

impl Key for UniversalPrivate {
    fn from_pem(pem: &str) -> Result<Self, KeyErr> {
        if let Ok(key) = rsa::pkcs1v15::SigningKey::from_pkcs8_pem(pem) {
            return Ok(Self::RSA(key));
        }
        if let Ok(key) = ed25519_dalek::SigningKey::from_pkcs8_pem(pem) {
            return Ok(Self::Ed35519(key));
        }
        Err(KeyErr::ParseFailure(
            "did not parse as rsa or ed2559".to_string(),
        ))
    }

    fn to_pem(&self) -> Result<String, KeyErr> {
        match self {
            UniversalPrivate::RSA(rsa_private_key) => {
                match rsa_private_key.to_pkcs8_pem(LINE_ENDING) {
                    Ok(pem) => Ok(pem.to_string()),
                    Err(_) => todo!(),
                }
            }
            UniversalPrivate::Ed35519(signing_key) => match signing_key.to_pkcs8_pem(LINE_ENDING) {
                Ok(pem) => Ok(pem.to_string()),
                Err(_) => todo!(),
            },
        }
    }
}

impl PrivateKey for UniversalPrivate {
    fn sign(&mut self, content: &[u8]) -> String {
        match self {
            UniversalPrivate::RSA(rsa_private_key) => {
                rsa_private_key.sign(content).to_string()
            },
            UniversalPrivate::Ed35519(signing_key) => {
                signing_key.sign(content).to_string()
            },
        }
    }

    fn generate(algorithm: Algorithms) -> Self {
        match algorithm {
            Algorithms::RsaSha256 => todo!(),
            Algorithms::Hs2019 => {
                let mut csprng = OsRng;
                let signing_key: ed25519_dalek::SigningKey =
                    ed25519_dalek::SigningKey::generate(&mut csprng);
                Self::Ed35519(signing_key)
            }
        }
    }

    fn public_key_pem(&self) -> Result<String, KeyErr> {
        match self {
            UniversalPrivate::RSA(rsa_private_key) => {
                match rsa_private_key
                    .verifying_key()
                    .to_public_key_pem(LINE_ENDING)
                {
                    Ok(pem) => Ok(pem),
                    Err(_) => todo!(),
                }
            }
            UniversalPrivate::Ed35519(signing_key) => {
                let verifying_key = signing_key.verifying_key();
                match ed25519_dalek::pkcs8::EncodePublicKey::to_public_key_pem(
                    &verifying_key,
                    LINE_ENDING,
                ) {
                    Ok(pem) => Ok(pem),
                    Err(_) => todo!(),
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum UniversalPublic {
    RSA(rsa::pkcs1v15::VerifyingKey<Sha256>),
    Ed35519(ed25519_dalek::VerifyingKey),
}

impl Key for UniversalPublic {
    fn from_pem(pem: &str) -> Result<Self, KeyErr> {
        if let Ok(key) = rsa::pkcs1v15::VerifyingKey::from_public_key_pem(pem) {
            return Ok(Self::RSA(key));
        }
        if let Ok(key) = VerifyingKey::from_public_key_pem(pem) {
            return Ok(Self::Ed35519(key));
        }
        Err(KeyErr::ParseFailure(
            "did not parse as rsa or ed2559".to_string(),
        ))
    }

    fn to_pem(&self) -> Result<String, KeyErr> {
        match self {
            UniversalPublic::RSA(rsa_public_key) => {
                match rsa_public_key.to_public_key_pem(LINE_ENDING) {
                    Ok(pem) => Ok(pem),
                    Err(_) => todo!(),
                }
            }
            UniversalPublic::Ed35519(verifying_key) => {
                match ed25519_dalek::pkcs8::EncodePublicKey::to_public_key_pem(
                    verifying_key,
                    LINE_ENDING,
                ) {
                    Ok(pem) => Ok(pem),
                    Err(_) => todo!(),
                }
            }
        }
    }
}

impl PublicKey for UniversalPublic {
    fn verify(&self, plain_content: &[u8], signature: &[u8]) -> bool {
        match self {
            UniversalPublic::RSA(rsa_public_key) => {
                use rsa::signature::Verifier;

                let Ok(signature) = rsa::pkcs1v15::Signature::try_from(signature) else {
                    return false;
                };
                let verified = rsa_public_key
                    .verify(plain_content, &signature);
                verified.is_ok()
            }
            UniversalPublic::Ed35519(verifying_key) => {
                let Ok(signature) = signature.try_into() else {
                    return false;
                };
                let signature = ed25519_dalek::Signature::from_bytes(signature);
                let first = sha512_hash(plain_content);
                let verified = verifying_key.verify_strict(first.as_bytes(), &signature);
                if verified.is_ok() {
                    return true;
                }
                // we do this just in case its hashed with sha256 because things can be messey 
                // and we're just going to try our best to support everything
                let second = sha256_hash(plain_content);
                let verified = verifying_key.verify_strict(second.as_bytes(), &signature);
                verified.is_ok()
            }
        }
    }
}
