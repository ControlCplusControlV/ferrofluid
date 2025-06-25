use std::{error::Error, fmt, sync::Arc};

use alloy::primitives::{Address, B256};
use async_trait::async_trait;
use base64::{engine::general_purpose, Engine as _};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::signers::{HyperliquidSignature, HyperliquidSigner, SignerError};

const PRIVY_API: &str = "https://api.privy.io/v1";

/// Privy-specific errors
#[derive(Debug)]
pub enum PrivyError {
    Http(reqwest::Error),
    Api(StatusCode, String),
    Serde(serde_json::Error),
    Hex(hex::FromHexError),
    InvalidSignature,
    MissingEnvVar(String),
}

impl fmt::Display for PrivyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PrivyError::*;
        match self {
            Http(e) => write!(f, "network error: {e}"),
            Api(code, s) => write!(f, "privy error {code}: {s}"),
            Serde(e) => write!(f, "serde error: {e}"),
            Hex(e) => write!(f, "hex decode error: {e}"),
            InvalidSignature => write!(f, "cannot parse signature from response"),
            MissingEnvVar(var) => write!(f, "missing environment variable: {var}"),
        }
    }
}

impl Error for PrivyError {}

impl From<reqwest::Error> for PrivyError {
    fn from(e: reqwest::Error) -> Self {
        Self::Http(e)
    }
}

impl From<serde_json::Error> for PrivyError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}

impl From<hex::FromHexError> for PrivyError {
    fn from(e: hex::FromHexError) -> Self {
        Self::Hex(e)
    }
}

/// Privy signer implementation for Hyperliquid
#[derive(Clone)]
pub struct PrivySigner {
    client: Arc<Client>,
    wallet_id: String,
    address: Address,
    app_id: String,
    basic_auth: String,
}

impl PrivySigner {
    /// Create a new Privy signer
    /// Reads PRIVY_APP_ID and PRIVY_SECRET from environment variables
    pub fn new(wallet_id: String, address: Address) -> Result<Self, PrivyError> {
        let app_id = std::env::var("PRIVY_APP_ID")
            .map_err(|_| PrivyError::MissingEnvVar("PRIVY_APP_ID".to_string()))?;
        let secret = std::env::var("PRIVY_SECRET")
            .map_err(|_| PrivyError::MissingEnvVar("PRIVY_SECRET".to_string()))?;

        let creds = general_purpose::STANDARD.encode(format!("{app_id}:{secret}"));

        Ok(Self {
            client: Arc::new(Client::builder().build()?),
            wallet_id,
            address,
            app_id,
            basic_auth: format!("Basic {creds}"),
        })
    }

    /// Create a new Privy signer with explicit credentials
    pub fn with_credentials(
        wallet_id: String,
        address: Address,
        app_id: String,
        secret: String,
    ) -> Result<Self, PrivyError> {
        let creds = general_purpose::STANDARD.encode(format!("{app_id}:{secret}"));

        Ok(Self {
            client: Arc::new(Client::builder().build()?),
            wallet_id,
            address,
            app_id,
            basic_auth: format!("Basic {creds}"),
        })
    }

    /// Internal RPC helper
    async fn rpc<T: for<'de> Deserialize<'de>>(
        &self,
        body: Value,
    ) -> Result<T, PrivyError> {
        let url = format!("{PRIVY_API}/wallets/{}/rpc", self.wallet_id);
        let resp = self
            .client
            .post(url)
            .header("Authorization", &self.basic_auth)
            .header("privy-app-id", &self.app_id)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let txt = resp.text().await.unwrap_or_default();
            return Err(PrivyError::Api(status, txt));
        }

        Ok(resp.json::<T>().await?)
    }
}

#[derive(Deserialize)]
struct SignResponse {
    data: SignData,
}

#[derive(Deserialize)]
struct SignData {
    signature: String,
}

#[async_trait]
impl HyperliquidSigner for PrivySigner {
    async fn sign_hash(&self, hash: B256) -> Result<HyperliquidSignature, SignerError> {
        // Convert hash to hex string with 0x prefix
        let hash_hex = format!("0x{}", hex::encode(hash));

        // Use secp256k1_sign for raw hash signing
        let body = json!({
            "method": "secp256k1_sign",
            "params": {
                "hash": hash_hex
            }
        });

        let resp: SignResponse = self
            .rpc(body)
            .await
            .map_err(|e| SignerError::SigningFailed(e.to_string()))?;

        // Parse the signature string (0x-prefixed hex)
        let sig_hex = resp
            .data
            .signature
            .strip_prefix("0x")
            .unwrap_or(&resp.data.signature);

        let sig_bytes = hex::decode(sig_hex).map_err(|e| {
            SignerError::SigningFailed(format!("Invalid hex signature: {e}"))
        })?;

        if sig_bytes.len() != 65 {
            return Err(SignerError::SigningFailed(format!(
                "Invalid signature length: expected 65, got {}",
                sig_bytes.len()
            )));
        }

        // Extract r, s, v from the signature bytes
        let mut r_bytes = [0u8; 32];
        let mut s_bytes = [0u8; 32];
        r_bytes.copy_from_slice(&sig_bytes[0..32]);
        s_bytes.copy_from_slice(&sig_bytes[32..64]);
        let v = sig_bytes[64];

        // Convert v to EIP-155 format if needed
        let v = if v < 27 { v + 27 } else { v };

        Ok(HyperliquidSignature {
            r: alloy::primitives::U256::from_be_bytes(r_bytes),
            s: alloy::primitives::U256::from_be_bytes(s_bytes),
            v: v as u64,
        })
    }

    fn address(&self) -> Address {
        self.address
    }
}

#[cfg(test)]
mod tests {
    use alloy::primitives::address;

    use super::*;

    #[test]
    fn test_privy_signer_creation() {
        // This test would require actual Privy credentials
        // For now, just test that missing env vars return appropriate errors
        let result = PrivySigner::new(
            "test-wallet-id".to_string(),
            address!("0000000000000000000000000000000000000000"),
        );

        match result {
            Err(PrivyError::MissingEnvVar(var)) => {
                assert!(var == "PRIVY_APP_ID" || var == "PRIVY_SECRET");
            }
            _ => panic!("Expected MissingEnvVar error"),
        }
    }
}
