use anyhow::{anyhow, bail, Result};
use base64::{engine::general_purpose::STANDARD as engine, Engine as _};
use ed25519_consensus::{
    Signature as Ed25519Signature, SigningKey as Ed25519SigningKey,
    VerificationKey as Ed25519VerifyingKey,
};
use secp256k1::{
    ecdsa::Signature as Secp256k1Signature, Message as Secp256k1Message,
    PublicKey as Secp256k1VerifyingKey, SecretKey as Secp256k1SigningKey, SECP256K1,
};
use serde::{Deserialize, Serialize};
use std::{self};

use crate::digest::Digest;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Default)]
pub enum Signature {
    Secp256k1(Secp256k1Signature),
    Ed25519(Ed25519Signature),
    #[default]
    Placeholder,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
/// Represents a public key supported by the system.
pub enum VerifyingKey {
    /// Bitcoin, Ethereum
    Secp256k1(Secp256k1VerifyingKey),
    /// Cosmos, OpenSSH, GnuPG
    Ed25519(Ed25519VerifyingKey),
}

impl VerifyingKey {
    /// Returns the byte representation of the public key.
    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            VerifyingKey::Ed25519(vk) => vk.to_bytes().to_vec(),
            VerifyingKey::Secp256k1(vk) => vk.serialize().to_vec(),
        }
    }

    pub fn verify_signature(&self, message: &[u8], signature: &Signature) -> Result<()> {
        match self {
            VerifyingKey::Ed25519(vk) => {
                let Signature::Ed25519(signature) = signature else {
                    bail!("Invalid signature type");
                };

                vk.verify(signature, message)
                    .map_err(|e| anyhow!("Failed to verify signature: {}", e))
            }
            VerifyingKey::Secp256k1(vk) => {
                let Signature::Secp256k1(signature) = signature else {
                    bail!("Invalid signature type");
                };
                let hashed_message = Digest::hash(message).to_bytes();
                let message = Secp256k1Message::from_digest(hashed_message);
                vk.verify(SECP256K1, &message, signature)
                    .map_err(|e| anyhow!("Failed to verify signature: {}", e))
            }
        }
    }
}

impl From<Ed25519SigningKey> for VerifyingKey {
    fn from(sk: Ed25519SigningKey) -> Self {
        VerifyingKey::Ed25519(sk.verification_key())
    }
}

impl From<Ed25519VerifyingKey> for VerifyingKey {
    fn from(vk: Ed25519VerifyingKey) -> Self {
        VerifyingKey::Ed25519(vk)
    }
}

impl From<Secp256k1SigningKey> for VerifyingKey {
    fn from(sk: Secp256k1SigningKey) -> Self {
        sk.public_key(SECP256K1).into()
    }
}

impl From<Secp256k1VerifyingKey> for VerifyingKey {
    fn from(vk: Secp256k1VerifyingKey) -> Self {
        VerifyingKey::Secp256k1(vk)
    }
}

impl TryFrom<String> for VerifyingKey {
    type Error = anyhow::Error;

    /// Attempts to create a `VerifyingKey` from a base64-encoded string.
    ///
    /// # Arguments
    ///
    /// * `s` - The base64-encoded string representation of the public key.
    ///
    /// Depending on the length of the input string, the function will attempt to
    /// decode it and create a `VerifyingKey` instance. According to the specifications,
    /// the input string should be either [32 bytes (Ed25519)](https://datatracker.ietf.org/doc/html/rfc8032#section-5.1.5) or [33/65 bytes (Secp256k1)](https://www.secg.org/sec1-v2.pdf).
    /// The secp256k1 key can be either compressed (33 bytes) or uncompressed (65 bytes).
    ///
    /// # Returns
    ///
    /// * `Ok(VerifyingKey)` if the conversion was successful.
    /// * `Err` if the input is invalid or the conversion failed.
    fn try_from(s: String) -> std::result::Result<Self, Self::Error> {
        let bytes =
            engine.decode(s).map_err(|e| anyhow!("Failed to decode base64 string: {}", e))?;

        match bytes.len() {
            32 => {
                let vk = Ed25519VerifyingKey::try_from(bytes.as_slice())
                    .map_err(|e| anyhow!("Invalid Ed25519 key: {}", e))?;
                Ok(VerifyingKey::Ed25519(vk))
            }
            33 | 65 => {
                let vk = Secp256k1VerifyingKey::from_slice(bytes.as_slice())
                    .map_err(|e| anyhow!("Invalid Secp256k1 key: {}", e))?;
                Ok(VerifyingKey::Secp256k1(vk))
            }
            _ => Err(anyhow!("Invalid public key length")),
        }
    }
}

impl std::fmt::Display for VerifyingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let encoded = engine.encode(self.as_bytes());
        write!(f, "{}", encoded)
    }
}

#[derive(Clone)]
pub enum SigningKey {
    Ed25519(Box<Ed25519SigningKey>),
    Secp256k1(Secp256k1SigningKey),
}

impl SigningKey {
    pub fn sign(&self, message: &[u8]) -> Signature {
        match self {
            SigningKey::Ed25519(sk) => Signature::Ed25519(sk.sign(message)),
            SigningKey::Secp256k1(sk) => {
                let hashed_message = Digest::hash(message).to_bytes();
                let message = Secp256k1Message::from_digest(hashed_message);
                let signature = SECP256K1.sign_ecdsa(&message, sk);
                Signature::Secp256k1(signature)
            }
        }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        match self {
            SigningKey::Ed25519(sk) => sk.verification_key().into(),
            SigningKey::Secp256k1(sk) => sk.public_key(SECP256K1).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::OsRng;

    #[test]
    fn test_verifying_key_from_string_ed25519() {
        let original_key =
            SigningKey::Ed25519(Box::new(Ed25519SigningKey::new(OsRng))).verifying_key();
        let encoded = engine.encode(original_key.as_bytes());

        let result = VerifyingKey::try_from(encoded);
        assert!(result.is_ok());

        let decoded_key = result.unwrap();
        assert_eq!(decoded_key.as_bytes(), original_key.as_bytes());
    }

    #[test]
    fn test_verifying_key_from_string_secp256k1() {
        let original_key =
            SigningKey::Secp256k1(Secp256k1SigningKey::new(&mut OsRng)).verifying_key();
        let encoded = engine.encode(original_key.as_bytes());

        let result = VerifyingKey::try_from(encoded);
        assert!(result.is_ok());

        let decoded_key = result.unwrap();
        assert_eq!(decoded_key.as_bytes(), original_key.as_bytes());
    }

    #[test]
    fn test_verifying_key_from_string_invalid_length() {
        let invalid_bytes: [u8; 31] = [1; 31];
        let encoded = engine.encode(invalid_bytes);

        let result = VerifyingKey::try_from(encoded);
        assert!(result.is_err());
    }
}
