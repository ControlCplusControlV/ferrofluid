/// Macro for user actions that use HyperliquidSignTransaction domain
/// All user actions must have signature_chain_id as their first field
#[macro_export]
macro_rules! hyperliquid_action {
    (
        $(#[$meta:meta])*
        struct $name:ident {
            pub signature_chain_id: u64,
            $(
                $(#[$field_meta:meta])*
                pub $field:ident: $type:ty
            ),* $(,)?
        }
        => $type_string:literal
        => encode($($encode_field:ident),* $(,)?)
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            pub signature_chain_id: u64,
            $(
                $(#[$field_meta])*
                pub $field: $type,
            )*
        }

        impl $crate::types::eip712::HyperliquidAction for $name {
            const TYPE_STRING: &'static str = $type_string;
            const USE_PREFIX: bool = true;

            fn chain_id(&self) -> Option<u64> {
                Some(self.signature_chain_id)
            }

            fn encode_data(&self) -> Vec<u8> {
                let mut encoded = Vec::new();
                encoded.extend_from_slice(&Self::type_hash()[..]);
                $(
                    encoded.extend_from_slice(&$crate::types::eip712::encode_value(&self.$encode_field)[..]);
                )*
                encoded
            }
        }
    };
}

/// Macro for L1 actions that use the Exchange domain
#[macro_export]
macro_rules! l1_action {
    (
        $(#[$meta:meta])*
        struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                pub $field:ident: $type:ty
            ),* $(,)?
        }
        => $type_string:literal
        => encode($($encode_field:ident),* $(,)?)
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            $(
                $(#[$field_meta])*
                pub $field: $type,
            )*
        }

        impl $crate::types::eip712::HyperliquidAction for $name {
            const TYPE_STRING: &'static str = $type_string;
            const USE_PREFIX: bool = false;

            // L1 actions use the Exchange domain with chain ID 1337
            fn domain(&self) -> alloy_sol_types::Eip712Domain {
                alloy_sol_types::eip712_domain! {
                    name: "Exchange",
                    version: "1",
                    chain_id: 1337u64,
                    verifying_contract: alloy_primitives::address!("0000000000000000000000000000000000000000"),
                }
            }

            fn encode_data(&self) -> Vec<u8> {
                let mut encoded = Vec::new();
                encoded.extend_from_slice(&Self::type_hash()[..]);
                $(
                    encoded.extend_from_slice(&$crate::types::eip712::encode_value(&self.$encode_field)[..]);
                )*
                encoded
            }
        }
    };
}
