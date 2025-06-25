use alloy::primitives::B256;
use serde;

use crate::l1_action;
use crate::types::requests::{
    BuilderInfo, CancelRequest, CancelRequestCloid, ModifyRequest, OrderRequest,
};

// User Actions (with HyperliquidTransaction: prefix)

// UsdSend needs custom serialization for signature_chain_id
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsdSend {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub amount: String,
    pub time: u64,
}

impl crate::types::eip712::HyperliquidAction for UsdSend {
    const TYPE_STRING: &'static str =
        "UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)";
    const USE_PREFIX: bool = true;

    fn chain_id(&self) -> Option<u64> {
        Some(self.signature_chain_id)
    }

    fn encode_data(&self) -> Vec<u8> {
        use crate::types::eip712::encode_value;
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        encoded.extend_from_slice(&encode_value(&self.hyperliquid_chain)[..]);
        encoded.extend_from_slice(&encode_value(&self.destination)[..]);
        encoded.extend_from_slice(&encode_value(&self.amount)[..]);
        encoded.extend_from_slice(&encode_value(&self.time)[..]);
        encoded
    }
}

// Withdraw needs custom serialization for signature_chain_id
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Withdraw {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub amount: String,
    pub time: u64,
}

impl crate::types::eip712::HyperliquidAction for Withdraw {
    const TYPE_STRING: &'static str =
        "Withdraw(string hyperliquidChain,string destination,string amount,uint64 time)";
    const USE_PREFIX: bool = true;

    fn chain_id(&self) -> Option<u64> {
        Some(self.signature_chain_id)
    }

    fn encode_data(&self) -> Vec<u8> {
        use crate::types::eip712::encode_value;
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        encoded.extend_from_slice(&encode_value(&self.hyperliquid_chain)[..]);
        encoded.extend_from_slice(&encode_value(&self.destination)[..]);
        encoded.extend_from_slice(&encode_value(&self.amount)[..]);
        encoded.extend_from_slice(&encode_value(&self.time)[..]);
        encoded
    }
}

// SpotSend needs custom serialization for signature_chain_id
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotSend {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub destination: String,
    pub token: String,
    pub amount: String,
    pub time: u64,
}

impl crate::types::eip712::HyperliquidAction for SpotSend {
    const TYPE_STRING: &'static str = "SpotSend(string hyperliquidChain,string destination,string token,string amount,uint64 time)";
    const USE_PREFIX: bool = true;

    fn chain_id(&self) -> Option<u64> {
        Some(self.signature_chain_id)
    }

    fn encode_data(&self) -> Vec<u8> {
        use crate::types::eip712::encode_value;
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        encoded.extend_from_slice(&encode_value(&self.hyperliquid_chain)[..]);
        encoded.extend_from_slice(&encode_value(&self.destination)[..]);
        encoded.extend_from_slice(&encode_value(&self.token)[..]);
        encoded.extend_from_slice(&encode_value(&self.amount)[..]);
        encoded.extend_from_slice(&encode_value(&self.time)[..]);
        encoded
    }
}

// ApproveAgent needs custom serialization for the address field
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveAgent {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    #[serde(serialize_with = "serialize_address")]
    pub agent_address: alloy::primitives::Address,
    pub agent_name: Option<String>,
    pub nonce: u64,
}

pub(crate) fn serialize_address<S>(
    address: &alloy::primitives::Address,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&format!("{address:#x}"))
}

pub(crate) fn serialize_chain_id<S>(
    chain_id: &u64,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    // Serialize as hex string to match SDK format
    serializer.serialize_str(&format!("{chain_id:#x}"))
}

impl crate::types::eip712::HyperliquidAction for ApproveAgent {
    const TYPE_STRING: &'static str = "ApproveAgent(string hyperliquidChain,address agentAddress,string agentName,uint64 nonce)";
    const USE_PREFIX: bool = true;

    fn chain_id(&self) -> Option<u64> {
        Some(self.signature_chain_id)
    }

    fn encode_data(&self) -> Vec<u8> {
        use crate::types::eip712::encode_value;
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        encoded.extend_from_slice(&encode_value(&self.hyperliquid_chain)[..]);
        encoded.extend_from_slice(&encode_value(&self.agent_address)[..]);
        // SDK uses unwrap_or_default() for agent_name
        let agent_name = self.agent_name.clone().unwrap_or_default();
        encoded.extend_from_slice(&encode_value(&agent_name)[..]);
        encoded.extend_from_slice(&encode_value(&self.nonce)[..]);
        encoded
    }
}

// ApproveBuilderFee needs custom serialization for signature_chain_id
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApproveBuilderFee {
    #[serde(serialize_with = "serialize_chain_id")]
    pub signature_chain_id: u64,
    pub hyperliquid_chain: String,
    pub max_fee_rate: String,
    pub builder: String,
    pub nonce: u64,
}

impl crate::types::eip712::HyperliquidAction for ApproveBuilderFee {
    const TYPE_STRING: &'static str = "ApproveBuilderFee(string hyperliquidChain,string maxFeeRate,string builder,uint64 nonce)";
    const USE_PREFIX: bool = true;

    fn chain_id(&self) -> Option<u64> {
        Some(self.signature_chain_id)
    }

    fn encode_data(&self) -> Vec<u8> {
        use crate::types::eip712::encode_value;
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        encoded.extend_from_slice(&encode_value(&self.hyperliquid_chain)[..]);
        encoded.extend_from_slice(&encode_value(&self.max_fee_rate)[..]);
        encoded.extend_from_slice(&encode_value(&self.builder)[..]);
        encoded.extend_from_slice(&encode_value(&self.nonce)[..]);
        encoded
    }
}

// L1 Actions (use Exchange domain)

l1_action! {
    /// Agent connection action
    struct Agent {
        pub source: String,
        pub connection_id: B256,
    }
    => "Agent(string source,bytes32 connectionId)"
    => encode(source, connection_id)
}

// Exchange Actions (these don't need EIP-712 signing but are included for completeness)

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLeverage {
    pub asset: u32,
    pub is_cross: bool,
    pub leverage: u32,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIsolatedMargin {
    pub asset: u32,
    pub is_buy: bool,
    pub ntli: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultTransfer {
    pub vault_address: String,
    pub is_deposit: bool,
    pub usd: u64,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotUser {
    pub class_transfer: ClassTransfer,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassTransfer {
    pub usd_size: u64,
    pub to_perp: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetReferrer {
    pub code: String,
}

// Bulk actions that contain other types

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkOrder {
    pub orders: Vec<OrderRequest>,
    pub grouping: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub builder: Option<BuilderInfo>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancel {
    pub cancels: Vec<CancelRequest>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkModify {
    pub modifies: Vec<ModifyRequest>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkCancelCloid {
    pub cancels: Vec<CancelRequestCloid>,
}

// Types are now imported from requests.rs

// The macros don't handle signature_chain_id, so we need to remove the duplicate trait impls

#[cfg(test)]
mod tests {
    use alloy::primitives::keccak256;

    use super::*;
    use crate::types::eip712::HyperliquidAction;

    #[test]
    fn test_usd_send_type_hash() {
        let expected = keccak256(
            "HyperliquidTransaction:UsdSend(string hyperliquidChain,string destination,string amount,uint64 time)",
        );
        assert_eq!(UsdSend::type_hash(), expected);
    }

    #[test]
    fn test_agent_type_hash() {
        // L1 actions don't use the HyperliquidTransaction: prefix
        let expected = keccak256("Agent(string source,bytes32 connectionId)");
        assert_eq!(Agent::type_hash(), expected);
    }

    #[test]
    fn test_agent_domain() {
        let agent = Agent {
            source: "a".to_string(),
            connection_id: B256::default(),
        };

        // L1 actions use the Exchange domain
        let domain = agent.domain();
        let expected_domain = alloy::sol_types::eip712_domain! {
            name: "Exchange",
            version: "1",
            chain_id: 1337u64,
            verifying_contract: alloy::primitives::address!("0000000000000000000000000000000000000000"),
        };

        // Compare domain separators to verify they're the same
        assert_eq!(domain.separator(), expected_domain.separator());
    }
}
