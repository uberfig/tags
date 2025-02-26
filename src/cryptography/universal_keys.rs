use super::key::{Algorithms, Key, KeyErr, PrivateKey, PublicKey};
use ed25519_dalek::VerifyingKey;
use rand::rngs::OsRng;
use rsa::{
    pkcs8::{DecodePublicKey, EncodePublicKey},
    RsaPrivateKey, RsaPublicKey,
};
#[derive(Debug, Clone)]
pub enum UniversalPrivate {
    RSA(RsaPrivateKey),
    Ed35519(ed25519_dalek::SigningKey),
}

impl Key for UniversalPrivate {
    fn from_pem(pem: &str) -> Result<Self, KeyErr> {
        // Ok(OpenSSLPrivate(PKey::private_key_from_pem(pem)?))
        todo!()
    }

    fn to_pem(&self) -> Result<String, KeyErr> {
        // let bytes = self.0.private_key_to_pem_pkcs8()?;
        // let pem = String::from_utf8(bytes)?;
        // Ok(pem)
        todo!()
    }
}

impl PrivateKey for UniversalPrivate {
    fn sign(&mut self, content: &str) -> String {
        match self {
            UniversalPrivate::RSA(rsa_private_key) => todo!(),
            UniversalPrivate::Ed35519(signing_key) => todo!(),
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
                    .to_public_key()
                    .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
                {
                    Ok(pem) => Ok(pem),
                    Err(_) => todo!(),
                }
            }
            UniversalPrivate::Ed35519(signing_key) => {
                let verifying_key = signing_key.verifying_key();
                match ed25519_dalek::pkcs8::EncodePublicKey::to_public_key_pem(
                    &verifying_key,
                    rsa::pkcs8::LineEnding::LF,
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
    RSA(RsaPublicKey),
    Ed35519(ed25519_dalek::VerifyingKey),
}

impl Key for UniversalPublic {
    fn from_pem(pem: &str) -> Result<Self, KeyErr> {
        if let Ok(key) = RsaPublicKey::from_public_key_pem(pem) {
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
                match rsa_public_key.to_public_key_pem(rsa::pkcs8::LineEnding::LF) {
                    Ok(pem) => Ok(pem),
                    Err(_) => todo!(),
                }
            }
            UniversalPublic::Ed35519(verifying_key) => {
                match ed25519_dalek::pkcs8::EncodePublicKey::to_public_key_pem(
                    verifying_key,
                    rsa::pkcs8::LineEnding::LF,
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
        // let mut verifier = openssl::sign::Verifier::new_without_digest(&self.0).unwrap();
        // verifier.verify_oneshot(signature, plain_content).unwrap()
        match self {
            UniversalPublic::RSA(rsa_public_key) => {
                todo!()
            }
            UniversalPublic::Ed35519(verifying_key) => {
                let Ok(signature) = signature.try_into() else {
                    return false;
                };
                let signature = ed25519_dalek::Signature::from_bytes(signature);
                let verified = verifying_key.verify_strict(plain_content, &signature);
                verified.is_ok()
            }
        }
    }
}
