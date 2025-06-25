//! Test for ApproveAgent EIP-712 signing

#[cfg(test)]
mod tests {
    use alloy::primitives::{address, keccak256};
    use ferrofluid::types::actions::ApproveAgent;
    use ferrofluid::types::eip712::HyperliquidAction;

    #[test]
    fn test_approve_agent_type_hash() {
        let expected = keccak256(
            "HyperliquidTransaction:ApproveAgent(string hyperliquidChain,address agentAddress,string agentName,uint64 nonce)"
        );
        assert_eq!(ApproveAgent::type_hash(), expected);
    }

    #[test]
    fn test_approve_agent_serialization() {
        let action = ApproveAgent {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            agent_address: address!("1234567890123456789012345678901234567890"),
            agent_name: Some("Test Agent".to_string()),
            nonce: 1234567890,
        };

        // Serialize to JSON
        let json = serde_json::to_string(&action).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Check that address is serialized as hex string
        assert_eq!(
            parsed["agentAddress"].as_str().unwrap(),
            "0x1234567890123456789012345678901234567890"
        );
        assert_eq!(parsed["hyperliquidChain"].as_str().unwrap(), "Testnet");
        assert_eq!(parsed["agentName"].as_str().unwrap(), "Test Agent");
        assert_eq!(parsed["nonce"].as_u64().unwrap(), 1234567890);
    }

    #[test]
    fn test_approve_agent_struct_hash() {
        let action = ApproveAgent {
            signature_chain_id: 421614,
            hyperliquid_chain: "Testnet".to_string(),
            agent_address: address!("0D1d9635D0640821d15e323ac8AdADfA9c111414"),
            agent_name: None,
            nonce: 1690393044548,
        };

        // Test that struct hash is computed
        let struct_hash = action.struct_hash();
        // Just verify it's not zero
        assert_ne!(struct_hash, alloy::primitives::B256::ZERO);
    }
}
