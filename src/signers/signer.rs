use alloy::{
    primitives::{Address, Parity, B256, U256},
    signers::Signer,
};
use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct HyperliquidSignature {
    pub r: U256,
    pub s: U256,
    pub v: u64,
}

#[async_trait]
pub trait HyperliquidSigner: Send + Sync {
    /// Sign a hash and return the signature
    async fn sign_hash(&self, hash: B256) -> Result<HyperliquidSignature, SignerError>;

    /// Get the address of this signer
    fn address(&self) -> Address;
}

#[derive(Debug, thiserror::Error)]
pub enum SignerError {
    #[error("signing failed: {0}")]
    SigningFailed(String),

    #[error("signer unavailable")]
    Unavailable,
}

pub struct AlloySigner<S: Signer> {
    pub inner: S,
}

// Direct implementation for PrivateKeySigner
#[async_trait]
impl HyperliquidSigner for alloy::signers::local::PrivateKeySigner {
    async fn sign_hash(&self, hash: B256) -> Result<HyperliquidSignature, SignerError> {
        let sig = <Self as Signer>::sign_hash(self, &hash)
            .await
            .map_err(|e| SignerError::SigningFailed(e.to_string()))?;

        // Convert Parity to v value (27 or 28)
        let v = match sig.v() {
            Parity::Eip155(v) => v,
            Parity::NonEip155(true) => 28,
            Parity::NonEip155(false) => 27,
            Parity::Parity(true) => 28,
            Parity::Parity(false) => 27,
        };

        Ok(HyperliquidSignature {
            r: sig.r(),
            s: sig.s(),
            v,
        })
    }

    fn address(&self) -> Address {
        self.address()
    }
}

#[async_trait]
impl<S> HyperliquidSigner for AlloySigner<S>
where
    S: Signer + Send + Sync,
{
    async fn sign_hash(&self, hash: B256) -> Result<HyperliquidSignature, SignerError> {
        let sig = self
            .inner
            .sign_hash(&hash)
            .await
            .map_err(|e| SignerError::SigningFailed(e.to_string()))?;

        // Convert Parity to v value (27 or 28)
        let v = match sig.v() {
            Parity::Eip155(v) => v,
            Parity::NonEip155(true) => 28,
            Parity::NonEip155(false) => 27,
            Parity::Parity(true) => 28,
            Parity::Parity(false) => 27,
        };

        Ok(HyperliquidSignature {
            r: sig.r(),
            s: sig.s(),
            v,
        })
    }

    fn address(&self) -> Address {
        self.inner.address()
    }
}

#[cfg(test)]
mod tests {
    use alloy::{primitives::b256, signers::local::PrivateKeySigner};

    use super::*;
    use crate::types::{Agent, HyperliquidAction, UsdSend, Withdraw};

    fn get_test_signer() -> AlloySigner<PrivateKeySigner> {
        let private_key =
            "e908f86dbb4d55ac876378565aafeabc187f6690f046459397b17d9b9a19688e";
        let signer = private_key.parse::<PrivateKeySigner>().unwrap();
        AlloySigner { inner: signer }
    }

    #[tokio::test]
    async fn test_sign_l1_action() -> Result<(), Box<dyn std::error::Error>> {
        let signer = get_test_signer();
        let connection_id =
            b256!("de6c4037798a4434ca03cd05f00e3b803126221375cd1e7eaaaf041768be06eb");

        // Create Agent action
        let agent = Agent {
            source: "a".to_string(),
            connection_id,
        };

        // L1 actions use the Exchange domain with chain ID 1337 - provided by l1_action! macro
        let domain = agent.domain();

        // Use the action's eip712_signing_hash method which handles everything
        let signing_hash = agent.eip712_signing_hash(&domain);

        let mainnet_sig = signer.sign_hash(signing_hash).await?;

        let expected_mainnet = "fa8a41f6a3fa728206df80801a83bcbfbab08649cd34d9c0bfba7c7b2f99340f53a00226604567b98a1492803190d65a201d6805e5831b7044f17fd530aec7841c";
        let actual = format!(
            "{:064x}{:064x}{:02x}",
            mainnet_sig.r, mainnet_sig.s, mainnet_sig.v
        );

        assert_eq!(actual, expected_mainnet);

        // Test testnet signature with source "b"
        let agent_testnet = Agent {
            source: "b".to_string(),
            connection_id,
        };

        let testnet_hash = agent_testnet.eip712_signing_hash(&agent_testnet.domain());
        let testnet_sig = signer.sign_hash(testnet_hash).await?;

        let expected_testnet = "1713c0fc661b792a50e8ffdd59b637b1ed172d9a3aa4d801d9d88646710fb74b33959f4d075a7ccbec9f2374a6da21ffa4448d58d0413a0d335775f680a881431c";
        let actual_testnet = format!(
            "{:064x}{:064x}{:02x}",
            testnet_sig.r, testnet_sig.s, testnet_sig.v
        );

        assert_eq!(actual_testnet, expected_testnet);

        Ok(())
    }

    #[tokio::test]
    async fn test_sign_usd_transfer_action() -> Result<(), Box<dyn std::error::Error>> {
        let signer = get_test_signer();

        // Create UsdSend action
        let usd_send = UsdSend {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0x0D1d9635D0640821d15e323ac8AdADfA9c111414".to_string(),
            amount: "1".to_string(),
            time: 1690393044548,
        };

        // Use the action's own domain method which uses signature_chain_id
        let domain = usd_send.domain();

        // Use the action's eip712_signing_hash method
        let signing_hash = usd_send.eip712_signing_hash(&domain);

        let sig = signer.sign_hash(signing_hash).await?;

        let expected = "214d507bbdaebba52fa60928f904a8b2df73673e3baba6133d66fe846c7ef70451e82453a6d8db124e7ed6e60fa00d4b7c46e4d96cb2bd61fd81b6e8953cc9d21b";
        let actual = format!("{:064x}{:064x}{:02x}", sig.r, sig.s, sig.v);

        assert_eq!(actual, expected);

        Ok(())
    }

    #[tokio::test]
    async fn test_sign_withdraw_action() -> Result<(), Box<dyn std::error::Error>> {
        let signer = get_test_signer();

        // Create Withdraw action
        let withdraw = Withdraw {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            destination: "0x0D1d9635D0640821d15e323ac8AdADfA9c111414".to_string(),
            amount: "1".to_string(),
            time: 1690393044548,
        };

        // Use the action's own domain method which uses signature_chain_id
        let domain = withdraw.domain();

        // Use the action's eip712_signing_hash method
        let signing_hash = withdraw.eip712_signing_hash(&domain);

        let sig = signer.sign_hash(signing_hash).await?;

        let expected = "b3172e33d2262dac2b4cb135ce3c167fda55dafa6c62213564ab728b9f9ba76b769a938e9f6d603dae7154c83bf5a4c3ebab81779dc2db25463a3ed663c82ae41c";
        let actual = format!("{:064x}{:064x}{:02x}", sig.r, sig.s, sig.v);

        assert_eq!(actual, expected);

        Ok(())
    }
}
