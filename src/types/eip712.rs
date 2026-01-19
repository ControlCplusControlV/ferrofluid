use alloy::primitives::{keccak256, Address, B256, U256};
use alloy::sol_types::Eip712Domain;

pub trait HyperliquidAction: Sized + serde::Serialize {
    /// The EIP-712 type string (without HyperliquidTransaction: prefix)
    const TYPE_STRING: &'static str;

    /// Whether this uses the HyperliquidTransaction: prefix
    const USE_PREFIX: bool = true;

    /// Get chain ID for domain construction
    /// Override this method for actions with signature_chain_id
    fn chain_id(&self) -> Option<u64> {
        None
    }

    /// Get the EIP-712 domain for this action
    fn domain(&self) -> Eip712Domain {
        let chain_id = self.chain_id().unwrap_or(1); // Default to mainnet
        alloy::sol_types::eip712_domain! {
            name: "HyperliquidSignTransaction",
            version: "1",
            chain_id: chain_id,
            verifying_contract: alloy::primitives::address!("0000000000000000000000000000000000000000"),
        }
    }

    fn type_hash() -> B256 {
        let type_string = if Self::USE_PREFIX {
            format!("HyperliquidTransaction:{}", Self::TYPE_STRING)
        } else {
            Self::TYPE_STRING.to_string()
        };
        keccak256(type_string.as_bytes())
    }

    /// Encode the struct data according to EIP-712 rules
    /// Default implementation - should be overridden for proper field ordering
    fn encode_data(&self) -> Vec<u8> {
        // This is a placeholder - each action should implement proper encoding
        // based on its TYPE_STRING field order
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&Self::type_hash()[..]);
        // Subclasses should implement the rest
        encoded
    }

    fn struct_hash(&self) -> B256 {
        keccak256(self.encode_data())
    }

    fn eip712_signing_hash(&self, domain: &Eip712Domain) -> B256 {
        let domain_separator = domain.separator();
        let struct_hash = self.struct_hash();

        let mut buf = Vec::with_capacity(66);
        buf.push(0x19);
        buf.push(0x01);
        buf.extend_from_slice(&domain_separator[..]);
        buf.extend_from_slice(&struct_hash[..]);

        keccak256(&buf)
    }
}

/// Encode a value according to EIP-712 rules
pub fn encode_value<T: EncodeEip712>(value: &T) -> [u8; 32] {
    value.encode_eip712()
}

/// Trait for types that can be encoded in EIP-712
pub trait EncodeEip712 {
    fn encode_eip712(&self) -> [u8; 32];
}

impl EncodeEip712 for String {
    fn encode_eip712(&self) -> [u8; 32] {
        keccak256(self.as_bytes()).into()
    }
}

impl EncodeEip712 for u64 {
    fn encode_eip712(&self) -> [u8; 32] {
        U256::from(*self).to_be_bytes::<32>()
    }
}

impl EncodeEip712 for B256 {
    fn encode_eip712(&self) -> [u8; 32] {
        (*self).into()
    }
}

impl EncodeEip712 for Address {
    fn encode_eip712(&self) -> [u8; 32] {
        let mut result = [0u8; 32];
        result[12..].copy_from_slice(self.as_slice());
        result
    }
}

impl<T: EncodeEip712> EncodeEip712 for Option<T> {
    fn encode_eip712(&self) -> [u8; 32] {
        match self {
            Some(v) => v.encode_eip712(),
            None => keccak256("".as_bytes()).into(), // Empty string hash for None
        }
    }
}
