use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use alloy::primitives::{keccak256, Address, B256};
use http_body_util::{BodyExt, Full};
use hyper::{body::Bytes, Method, Request};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use serde::Serialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    constants::*,
    errors::HyperliquidError,
    providers::order_tracker::{OrderStatus, OrderTracker, TrackedOrder},
    signers::{HyperliquidSignature, HyperliquidSigner},
    types::{
        actions::*, eip712::HyperliquidAction, requests::*,
        responses::ExchangeResponseStatus, Symbol,
    },
};

type Result<T> = std::result::Result<T, HyperliquidError>;

/// Format a float for use in API requests
/// Formats to 8 decimal places and removes trailing zeros
fn format_float_string(value: f64) -> String {
    let mut x = format!("{value:.8}");
    while x.ends_with('0') {
        x.pop();
    }
    if x.ends_with('.') {
        x.pop();
    }
    if x == "-0" {
        "0".to_string()
    } else {
        x
    }
}

pub struct RawExchangeProvider<S: HyperliquidSigner> {
    client: Client<HttpsConnector<HttpConnector>, Full<Bytes>>,
    endpoint: &'static str,
    rate_limiter: Arc<crate::providers::info::RateLimiter>,
    signer: S,
    vault_address: Option<Address>,
    agent: Option<Address>,
    builder: Option<Address>,
    order_tracker: Option<OrderTracker>,
}

impl<S: HyperliquidSigner> RawExchangeProvider<S> {
    // ==================== Helper Methods ====================

    pub(crate) fn infer_network(&self) -> (u64, &'static str) {
        if self.endpoint.contains("testnet") {
            (CHAIN_ID_TESTNET, AGENT_SOURCE_TESTNET)
        } else {
            (CHAIN_ID_MAINNET, AGENT_SOURCE_MAINNET)
        }
    }

    /// Get the configured builder address
    pub fn builder(&self) -> Option<Address> {
        self.builder
    }

    /// Enable order tracking for this exchange instance
    pub fn with_order_tracking(mut self) -> Self {
        self.order_tracker = Some(OrderTracker::new());
        self
    }

    // ==================== Order Tracking Methods ====================

    /// Get a tracked order by CLOID
    pub fn get_tracked_order(&self, cloid: &Uuid) -> Option<TrackedOrder> {
        self.order_tracker.as_ref()?.get_order(cloid)
    }

    /// Get all tracked orders
    pub fn get_all_tracked_orders(&self) -> Vec<TrackedOrder> {
        self.order_tracker
            .as_ref()
            .map(|tracker| tracker.get_all_orders())
            .unwrap_or_default()
    }

    /// Get orders by status
    pub fn get_orders_by_status(&self, status: &OrderStatus) -> Vec<TrackedOrder> {
        self.order_tracker
            .as_ref()
            .map(|tracker| tracker.get_orders_by_status(status))
            .unwrap_or_default()
    }

    /// Get pending orders
    pub fn get_pending_orders(&self) -> Vec<TrackedOrder> {
        self.order_tracker
            .as_ref()
            .map(|tracker| tracker.get_pending_orders())
            .unwrap_or_default()
    }

    /// Get submitted orders
    pub fn get_submitted_orders(&self) -> Vec<TrackedOrder> {
        self.order_tracker
            .as_ref()
            .map(|tracker| tracker.get_submitted_orders())
            .unwrap_or_default()
    }

    /// Get failed orders
    pub fn get_failed_orders(&self) -> Vec<TrackedOrder> {
        self.order_tracker
            .as_ref()
            .map(|tracker| tracker.get_failed_orders())
            .unwrap_or_default()
    }

    /// Clear tracked orders
    pub fn clear_tracked_orders(&self) {
        if let Some(tracker) = &self.order_tracker {
            tracker.clear();
        }
    }

    /// Get the number of tracked orders
    pub fn tracked_order_count(&self) -> usize {
        self.order_tracker
            .as_ref()
            .map(|tracker| tracker.len())
            .unwrap_or(0)
    }

    // ==================== Constructors ====================

    pub fn mainnet(signer: S) -> Self {
        Self::new(signer, EXCHANGE_ENDPOINT_MAINNET, None, None, None)
    }

    pub fn testnet(signer: S) -> Self {
        Self::new(signer, EXCHANGE_ENDPOINT_TESTNET, None, None, None)
    }

    pub fn mainnet_vault(signer: S, vault_address: Address) -> Self {
        Self::new(
            signer,
            EXCHANGE_ENDPOINT_MAINNET,
            Some(vault_address),
            None,
            None,
        )
    }

    pub fn testnet_vault(signer: S, vault_address: Address) -> Self {
        Self::new(
            signer,
            EXCHANGE_ENDPOINT_TESTNET,
            Some(vault_address),
            None,
            None,
        )
    }

    pub fn mainnet_agent(signer: S, agent_address: Address) -> Self {
        Self::new(
            signer,
            EXCHANGE_ENDPOINT_MAINNET,
            None,
            Some(agent_address),
            None,
        )
    }

    pub fn testnet_agent(signer: S, agent_address: Address) -> Self {
        Self::new(
            signer,
            EXCHANGE_ENDPOINT_TESTNET,
            None,
            Some(agent_address),
            None,
        )
    }

    // New builder-specific constructors
    pub fn mainnet_builder(signer: S, builder_address: Address) -> Self {
        Self::new(
            signer,
            EXCHANGE_ENDPOINT_MAINNET,
            None,
            None,
            Some(builder_address),
        )
    }

    pub fn testnet_builder(signer: S, builder_address: Address) -> Self {
        Self::new(
            signer,
            EXCHANGE_ENDPOINT_TESTNET,
            None,
            None,
            Some(builder_address),
        )
    }

    // Combined constructors
    pub fn mainnet_with_options(
        signer: S,
        vault: Option<Address>,
        agent: Option<Address>,
        builder: Option<Address>,
    ) -> Self {
        Self::new(signer, EXCHANGE_ENDPOINT_MAINNET, vault, agent, builder)
    }

    pub fn testnet_with_options(
        signer: S,
        vault: Option<Address>,
        agent: Option<Address>,
        builder: Option<Address>,
    ) -> Self {
        Self::new(signer, EXCHANGE_ENDPOINT_TESTNET, vault, agent, builder)
    }

    fn new(
        signer: S,
        endpoint: &'static str,
        vault_address: Option<Address>,
        agent: Option<Address>,
        builder: Option<Address>,
    ) -> Self {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_only()
            .enable_http1()
            .build();
        let client = Client::builder(hyper_util::rt::TokioExecutor::new()).build(https);
        let rate_limiter = Arc::new(crate::providers::info::RateLimiter::new(
            RATE_LIMIT_MAX_TOKENS,
            RATE_LIMIT_REFILL_RATE,
        ));

        Self {
            client,
            endpoint,
            rate_limiter,
            signer,
            vault_address,
            agent,
            builder,
            order_tracker: None,
        }
    }

    // ==================== Direct Order Operations ====================

    pub async fn place_order(
        &self,
        order: &OrderRequest,
    ) -> Result<ExchangeResponseStatus> {
        self.rate_limiter.check_weight(WEIGHT_PLACE_ORDER)?;

        // Auto-generate CLOID if tracking is enabled and order doesn't have one
        let mut order = order.clone();
        let cloid = if let Some(tracker) = &self.order_tracker {
            let cloid = order
                .cloid
                .as_ref()
                .and_then(|c| Uuid::parse_str(c).ok())
                .unwrap_or_else(Uuid::new_v4);

            // Ensure the order has a cloid
            if order.cloid.is_none() {
                order = order.with_cloid(Some(cloid));
            }

            // Track the order
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            tracker.track_order(cloid, order.clone(), timestamp);

            Some(cloid)
        } else {
            order.cloid.as_ref().and_then(|c| Uuid::parse_str(c).ok())
        };

        let bulk_order = BulkOrder {
            orders: vec![order],
            grouping: "na".to_string(),
            builder: self.builder.map(|addr| BuilderInfo {
                builder: format!("0x{}", hex::encode(addr)),
                fee: 0, // Default fee, use place_order_with_builder_fee to specify
            }),
        };

        let result = self.send_l1_action("order", &bulk_order).await;

        // Update tracking status based on result
        if let Some(tracker) = &self.order_tracker {
            if let Some(cloid) = cloid {
                match &result {
                    Ok(response) => {
                        tracker.update_order_status(
                            &cloid,
                            OrderStatus::Submitted,
                            Some(response.clone()),
                        );
                    }
                    Err(e) => {
                        tracker.update_order_status(
                            &cloid,
                            OrderStatus::Failed(e.to_string()),
                            None,
                        );
                    }
                }
            }
        }

        result
    }

    pub async fn place_order_with_builder_fee(
        &self,
        order: &OrderRequest,
        builder_fee: u64,
    ) -> Result<ExchangeResponseStatus> {
        self.rate_limiter.check_weight(WEIGHT_PLACE_ORDER)?;

        // Auto-generate CLOID if tracking is enabled and order doesn't have one
        let mut order = order.clone();
        let cloid = if let Some(tracker) = &self.order_tracker {
            let cloid = order
                .cloid
                .as_ref()
                .and_then(|c| Uuid::parse_str(c).ok())
                .unwrap_or_else(Uuid::new_v4);

            // Ensure the order has a cloid
            if order.cloid.is_none() {
                order = order.with_cloid(Some(cloid));
            }

            // Track the order
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            tracker.track_order(cloid, order.clone(), timestamp);

            Some(cloid)
        } else {
            order.cloid.as_ref().and_then(|c| Uuid::parse_str(c).ok())
        };

        let bulk_order = BulkOrder {
            orders: vec![order],
            grouping: "na".to_string(),
            builder: self.builder.map(|addr| BuilderInfo {
                builder: format!("0x{}", hex::encode(addr)),
                fee: builder_fee,
            }),
        };

        let result = self.send_l1_action("order", &bulk_order).await;

        // Update tracking status based on result
        if let Some(tracker) = &self.order_tracker {
            if let Some(cloid) = cloid {
                match &result {
                    Ok(response) => {
                        tracker.update_order_status(
                            &cloid,
                            OrderStatus::Submitted,
                            Some(response.clone()),
                        );
                    }
                    Err(e) => {
                        tracker.update_order_status(
                            &cloid,
                            OrderStatus::Failed(e.to_string()),
                            None,
                        );
                    }
                }
            }
        }

        result
    }

    pub async fn place_order_with_cloid(
        &self,
        mut order: OrderRequest,
        cloid: Uuid,
    ) -> Result<ExchangeResponseStatus> {
        order = order.with_cloid(Some(cloid));
        // place_order will handle tracking with the provided cloid
        self.place_order(&order).await
    }

    pub async fn cancel_order(
        &self,
        asset: u32,
        oid: u64,
    ) -> Result<ExchangeResponseStatus> {
        self.rate_limiter.check_weight(WEIGHT_CANCEL_ORDER)?;

        let bulk_cancel = BulkCancel {
            cancels: vec![CancelRequest { asset, oid }],
        };

        self.send_l1_action("cancel", &bulk_cancel).await
    }

    pub async fn cancel_order_by_cloid(
        &self,
        asset: u32,
        cloid: Uuid,
    ) -> Result<ExchangeResponseStatus> {
        self.rate_limiter.check_weight(WEIGHT_CANCEL_ORDER)?;

        let bulk_cancel = BulkCancelCloid {
            cancels: vec![CancelRequestCloid::new(asset, cloid)],
        };

        self.send_l1_action("cancelByCloid", &bulk_cancel).await
    }

    pub async fn modify_order(
        &self,
        oid: u64,
        new_order: OrderRequest,
    ) -> Result<ExchangeResponseStatus> {
        self.rate_limiter.check_weight(WEIGHT_MODIFY_ORDER)?;

        let bulk_modify = BulkModify {
            modifies: vec![ModifyRequest {
                oid,
                order: new_order,
            }],
        };

        self.send_l1_action("batchModify", &bulk_modify).await
    }

    // ==================== Bulk Operations ====================

    pub async fn bulk_orders(
        &self,
        orders: Vec<OrderRequest>,
    ) -> Result<ExchangeResponseStatus> {
        self.rate_limiter.check_weight(WEIGHT_BULK_ORDER)?;

        let bulk_order = BulkOrder {
            orders,
            grouping: "na".to_string(),
            builder: self.builder.map(|addr| BuilderInfo {
                builder: format!("0x{}", hex::encode(addr)),
                fee: 0, // Default fee, use bulk_orders_with_builder_fee to specify
            }),
        };

        self.send_l1_action("order", &bulk_order).await
    }

    pub async fn bulk_orders_with_builder_fee(
        &self,
        orders: Vec<OrderRequest>,
        builder_fee: u64,
    ) -> Result<ExchangeResponseStatus> {
        self.rate_limiter.check_weight(WEIGHT_BULK_ORDER)?;

        let bulk_order = BulkOrder {
            orders,
            grouping: "na".to_string(),
            builder: self.builder.map(|addr| BuilderInfo {
                builder: format!("0x{}", hex::encode(addr)),
                fee: builder_fee,
            }),
        };

        self.send_l1_action("order", &bulk_order).await
    }

    pub async fn bulk_orders_with_cloids(
        &self,
        orders: Vec<(OrderRequest, Option<Uuid>)>,
    ) -> Result<ExchangeResponseStatus> {
        let orders = orders
            .into_iter()
            .map(|(order, cloid)| order.with_cloid(cloid))
            .collect();

        self.bulk_orders(orders).await
    }

    pub async fn bulk_cancel(
        &self,
        cancels: Vec<CancelRequest>,
    ) -> Result<ExchangeResponseStatus> {
        self.rate_limiter.check_weight(WEIGHT_BULK_CANCEL)?;

        let bulk_cancel = BulkCancel { cancels };
        self.send_l1_action("cancel", &bulk_cancel).await
    }

    pub async fn bulk_cancel_by_cloid(
        &self,
        cancels: Vec<CancelRequestCloid>,
    ) -> Result<ExchangeResponseStatus> {
        self.rate_limiter.check_weight(WEIGHT_BULK_CANCEL)?;

        let bulk_cancel = BulkCancelCloid { cancels };
        self.send_l1_action("cancelByCloid", &bulk_cancel).await
    }

    pub async fn bulk_modify(
        &self,
        modifies: Vec<ModifyRequest>,
    ) -> Result<ExchangeResponseStatus> {
        self.rate_limiter.check_weight(WEIGHT_BULK_ORDER)?;

        let bulk_modify = BulkModify { modifies };
        self.send_l1_action("batchModify", &bulk_modify).await
    }

    // ==================== Account Management ====================

    pub async fn update_leverage(
        &self,
        asset: u32,
        is_cross: bool,
        leverage: u32,
    ) -> Result<ExchangeResponseStatus> {
        let update = UpdateLeverage {
            asset,
            is_cross,
            leverage,
        };
        self.send_l1_action("updateLeverage", &update).await
    }

    pub async fn update_isolated_margin(
        &self,
        asset: u32,
        is_buy: bool,
        ntli: i64,
    ) -> Result<ExchangeResponseStatus> {
        let update = UpdateIsolatedMargin {
            asset,
            is_buy,
            ntli,
        };
        self.send_l1_action("updateIsolatedMargin", &update).await
    }

    pub async fn set_referrer(&self, code: String) -> Result<ExchangeResponseStatus> {
        let referrer = SetReferrer { code };
        self.send_l1_action("setReferrer", &referrer).await
    }

    // ==================== User Actions (EIP-712) ====================

    pub async fn usd_transfer(
        &self,
        destination: Address,
        amount: &str,
    ) -> Result<ExchangeResponseStatus> {
        let (chain_id, _) = self.infer_network();
        let chain = if chain_id == CHAIN_ID_MAINNET {
            "Mainnet"
        } else {
            "Testnet"
        };

        let action = UsdSend {
            signature_chain_id: chain_id,
            hyperliquid_chain: chain.to_string(),
            destination: format!("{destination:#x}"),
            amount: amount.to_string(),
            time: Self::current_nonce(),
        };

        self.send_user_action(&action).await
    }

    pub async fn withdraw(
        &self,
        destination: Address,
        amount: &str,
    ) -> Result<ExchangeResponseStatus> {
        let (chain_id, _) = self.infer_network();
        let chain = if chain_id == CHAIN_ID_MAINNET {
            "Mainnet"
        } else {
            "Testnet"
        };

        let action = Withdraw {
            signature_chain_id: chain_id,
            hyperliquid_chain: chain.to_string(),
            destination: format!("{destination:#x}"),
            amount: amount.to_string(),
            time: Self::current_nonce(),
        };

        self.send_user_action(&action).await
    }

    pub async fn spot_transfer(
        &self,
        destination: Address,
        token: impl Into<Symbol>,
        amount: &str,
    ) -> Result<ExchangeResponseStatus> {
        let (chain_id, _) = self.infer_network();
        let chain = if chain_id == CHAIN_ID_MAINNET {
            "Mainnet"
        } else {
            "Testnet"
        };

        let symbol = token.into();
        let action = SpotSend {
            signature_chain_id: chain_id,
            hyperliquid_chain: chain.to_string(),
            destination: format!("{destination:#x}"),
            token: symbol.as_str().to_string(),
            amount: amount.to_string(),
            time: Self::current_nonce(),
        };

        self.send_user_action(&action).await
    }

    pub async fn approve_agent(
        &self,
        agent_address: Address,
        agent_name: Option<String>,
    ) -> Result<ExchangeResponseStatus> {
        let (chain_id, _) = self.infer_network();
        let chain = if chain_id == CHAIN_ID_MAINNET {
            "Mainnet"
        } else {
            "Testnet"
        };

        let action = ApproveAgent {
            signature_chain_id: 421614, // Always use Arbitrum Sepolia chain ID like SDK
            hyperliquid_chain: chain.to_string(),
            agent_address,
            agent_name,
            nonce: Self::current_nonce(),
        };

        self.send_user_action(&action).await
    }

    /// Approve a new agent, generating a random key like the original SDK
    /// Returns (private_key_hex, response)
    pub async fn approve_agent_new(&self) -> Result<(String, ExchangeResponseStatus)> {
        use alloy::primitives::B256;
        use alloy::signers::local::PrivateKeySigner;
        use rand::Rng;

        // Generate random key
        let mut rng = rand::thread_rng();
        let mut key_bytes = [0u8; 32];
        rng.fill(&mut key_bytes);
        let key_hex = hex::encode(key_bytes);

        // Create a signer from the key to get the address
        let signer =
            PrivateKeySigner::from_bytes(&B256::from(key_bytes)).map_err(|e| {
                HyperliquidError::InvalidRequest(format!("Failed to create signer: {e}"))
            })?;
        let agent_address = signer.address();

        // Get chain info
        let (_, _) = self.infer_network();
        let chain = if self.endpoint.contains("testnet") {
            "Testnet"
        } else {
            "Mainnet"
        };

        // Create the action with proper Address type
        let action = ApproveAgent {
            signature_chain_id: 421614, // Always use Arbitrum Sepolia chain ID
            hyperliquid_chain: chain.to_string(),
            agent_address,
            agent_name: None,
            nonce: Self::current_nonce(),
        };

        // Use send_user_action which handles EIP-712 signing
        let response = self.send_user_action(&action).await?;

        Ok((key_hex, response))
    }

    pub async fn approve_builder_fee(
        &self,
        builder: Address,
        max_fee_rate: String,
    ) -> Result<ExchangeResponseStatus> {
        let (chain_id, _) = self.infer_network();
        let chain = if chain_id == CHAIN_ID_MAINNET {
            "Mainnet"
        } else {
            "Testnet"
        };

        let action = ApproveBuilderFee {
            signature_chain_id: chain_id,
            hyperliquid_chain: chain.to_string(),
            builder: format!("0x{}", hex::encode(builder)),
            max_fee_rate,
            nonce: Self::current_nonce(),
        };

        self.send_user_action(&action).await
    }

    // ==================== Vault Operations ====================

    pub async fn vault_transfer(
        &self,
        vault_address: Address,
        is_deposit: bool,
        usd: u64,
    ) -> Result<ExchangeResponseStatus> {
        let transfer = VaultTransfer {
            vault_address: format!("0x{}", hex::encode(vault_address)),
            is_deposit,
            usd,
        };

        self.send_l1_action("vaultTransfer", &transfer).await
    }

    // ==================== Spot Operations ====================

    pub async fn spot_transfer_to_perp(
        &self,
        usd_size: u64,
        to_perp: bool,
    ) -> Result<ExchangeResponseStatus> {
        let transfer = ClassTransfer { usd_size, to_perp };

        let spot_user = SpotUser {
            class_transfer: transfer,
        };

        self.send_l1_action("spotUser", &spot_user).await
    }

    // ==================== Helper Methods ====================

    fn current_nonce() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    fn hash_action<T: Serialize>(
        action_type: &str,
        action: &T,
        timestamp: u64,
        vault_address: Option<Address>,
    ) -> Result<B256> {
        // Create an enum wrapper for proper serialization
        // This matches how the original Hyperliquid SDK serializes actions
        // The enum variant becomes the "type" field in the serialized output
        #[derive(serde::Serialize)]
        #[serde(tag = "type")]
        #[serde(rename_all = "camelCase")]
        enum ActionWrapper<'a, T> {
            Order(&'a T),
            Cancel(&'a T),
            CancelByCloid(&'a T),
            BatchModify(&'a T),
            UpdateLeverage(&'a T),
            UpdateIsolatedMargin(&'a T),
            UsdSend(&'a T),
            SpotSend(&'a T),
            SpotUser(&'a T),
            VaultTransfer(&'a T),
            SetReferrer(&'a T),
            ApproveAgent(&'a T),
            ApproveBuilderFee(&'a T),
            Withdraw3(&'a T),
        }

        // Wrap the action based on type
        let wrapped = match action_type {
            "order" => ActionWrapper::Order(action),
            "cancel" => ActionWrapper::Cancel(action),
            "cancelByCloid" => ActionWrapper::CancelByCloid(action),
            "batchModify" => ActionWrapper::BatchModify(action),
            "updateLeverage" => ActionWrapper::UpdateLeverage(action),
            "updateIsolatedMargin" => ActionWrapper::UpdateIsolatedMargin(action),
            "usdSend" => ActionWrapper::UsdSend(action),
            "spotSend" => ActionWrapper::SpotSend(action),
            "spotUser" => ActionWrapper::SpotUser(action),
            "vaultTransfer" => ActionWrapper::VaultTransfer(action),
            "setReferrer" => ActionWrapper::SetReferrer(action),
            "approveAgent" => ActionWrapper::ApproveAgent(action),
            "approveBuilderFee" => ActionWrapper::ApproveBuilderFee(action),
            "withdraw3" => ActionWrapper::Withdraw3(action),
            _ => {
                return Err(HyperliquidError::InvalidRequest(format!(
                    "Unknown action type: {action_type}"
                )))
            }
        };

        // NOTE: Hyperliquid uses MessagePack (rmp_serde) for action serialization
        // This is different from typical EVM systems that use RLP
        let mut bytes = rmp_serde::to_vec_named(&wrapped).map_err(|e| {
            HyperliquidError::InvalidRequest(format!("Failed to serialize action: {e}"))
        })?;
        bytes.extend(timestamp.to_be_bytes());
        if let Some(vault) = vault_address {
            bytes.push(1);
            bytes.extend(vault.as_slice());
        } else {
            bytes.push(0);
        }
        Ok(keccak256(bytes))
    }

    async fn send_l1_action<T: Serialize>(
        &self,
        action_type: &str,
        action: &T,
    ) -> Result<ExchangeResponseStatus> {
        let nonce = Self::current_nonce();
        let connection_id =
            Self::hash_action(action_type, action, nonce, self.vault_address)?;

        // Create Agent L1 action
        let (_, agent_source) = self.infer_network();
        let agent = Agent {
            source: agent_source.to_string(),
            connection_id,
        };

        // Sign using EIP-712
        let domain = agent.domain();
        let signing_hash = agent.eip712_signing_hash(&domain);
        let signature = self.signer.sign_hash(signing_hash).await?;

        // Build action value with type tag
        let mut action_value = serde_json::to_value(action)?;
        if let Value::Object(ref mut map) = action_value {
            map.insert("type".to_string(), json!(action_type));
        }

        // Wrap action if using agent
        let final_action = if let Some(agent_address) = &self.agent {
            let (_, agent_source) = self.infer_network();
            json!({
                "type": "agent",
                "agentAddress": format!("{:#x}", agent_address),
                "agentAction": action_value,
                "source": agent_source,
            })
        } else {
            action_value
        };

        self.post(final_action, signature, nonce).await
    }

    async fn send_user_action<T: HyperliquidAction + Serialize>(
        &self,
        action: &T,
    ) -> Result<ExchangeResponseStatus> {
        let domain = action.domain();
        let signing_hash = action.eip712_signing_hash(&domain);
        let signature = self.signer.sign_hash(signing_hash).await?;

        // Get action type from type name
        // This extracts "UsdSend" from "ferrofluid::types::actions::UsdSend"
        let action_type = std::any::type_name::<T>()
            .split("::")
            .last()
            .unwrap_or("Unknown");

        // Get action value and extract nonce
        let mut action_value = serde_json::to_value(action)?;
        let nonce = action_value
            .get("time")
            .or_else(|| action_value.get("nonce"))
            .and_then(|v| v.as_u64())
            .unwrap_or_else(Self::current_nonce);

        // For ApproveAgent, we need to use camelCase type name to match SDK
        let type_tag = match action_type {
            "ApproveAgent" => "approveAgent",
            "UsdSend" => "usdSend",
            "Withdraw" => "withdraw3",
            "SpotSend" => "spotSend",
            "ApproveBuilderFee" => "approveBuilderFee",
            _ => action_type,
        };

        // Add type tag
        if let Value::Object(ref mut map) = action_value {
            map.insert("type".to_string(), json!(type_tag));
        }

        // Send directly without L1 wrapping for user actions
        self.post(action_value, signature, nonce).await
    }

    async fn post(
        &self,
        action: Value,
        signature: HyperliquidSignature,
        nonce: u64,
    ) -> Result<ExchangeResponseStatus> {
        // Hyperliquid expects signature as an object with r, s, v fields
        // not as a concatenated hex string
        let payload = json!({
            "action": action,
            "signature": {
                "r": format!("0x{:064x}", signature.r),
                "s": format!("0x{:064x}", signature.s),
                "v": signature.v,
            },
            "nonce": nonce,
            "vaultAddress": self.vault_address,
        });

        let body = Full::new(Bytes::from(serde_json::to_vec(&payload)?));
        let request = Request::builder()
            .method(Method::POST)
            .uri(self.endpoint)
            .header("Content-Type", "application/json")
            .body(body)
            .map_err(|e| HyperliquidError::Network(e.to_string()))?;

        let response = self
            .client
            .request(request)
            .await
            .map_err(|e| HyperliquidError::Network(e.to_string()))?;
        let status = response.status();
        let body_bytes = response
            .into_body()
            .collect()
            .await
            .map_err(|e| HyperliquidError::Network(e.to_string()))?
            .to_bytes();

        // Always try to deserialize the response as ExchangeResponseStatus
        // The API returns this format even for error status codes
        serde_json::from_slice(&body_bytes).map_err(|e| {
            // If deserialization fails and we have an error status,
            // return the HTTP error with the body
            if !status.is_success() {
                let body_text = String::from_utf8_lossy(&body_bytes);
                HyperliquidError::Http {
                    status: status.as_u16(),
                    body: body_text.to_string(),
                }
            } else {
                HyperliquidError::InvalidResponse(format!(
                    "Failed to parse exchange response: {e}"
                ))
            }
        })
    }
}

// ==================== OrderBuilder Pattern ====================

pub struct OrderBuilder<'a, S: HyperliquidSigner> {
    provider: &'a RawExchangeProvider<S>,
    asset: u32,
    is_buy: Option<bool>,
    limit_px: Option<String>,
    sz: Option<String>,
    reduce_only: bool,
    order_type: Option<OrderType>,
    cloid: Option<Uuid>,
}

impl<'a, S: HyperliquidSigner> OrderBuilder<'a, S> {
    pub fn new(provider: &'a RawExchangeProvider<S>, asset: u32) -> Self {
        Self {
            provider,
            asset,
            is_buy: None,
            limit_px: None,
            sz: None,
            reduce_only: false,
            order_type: None,
            cloid: None,
        }
    }

    pub fn buy(mut self) -> Self {
        self.is_buy = Some(true);
        self
    }

    pub fn sell(mut self) -> Self {
        self.is_buy = Some(false);
        self
    }

    pub fn limit_px(mut self, price: impl ToString) -> Self {
        self.limit_px = Some(price.to_string());
        self
    }

    pub fn size(mut self, size: impl ToString) -> Self {
        self.sz = Some(size.to_string());
        self
    }

    pub fn reduce_only(mut self, reduce: bool) -> Self {
        self.reduce_only = reduce;
        self
    }

    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = Some(order_type);
        self
    }

    pub fn cloid(mut self, id: Uuid) -> Self {
        self.cloid = Some(id);
        self
    }

    // Convenience methods for common order types
    pub fn limit_buy(self, price: impl ToString, size: impl ToString) -> Self {
        self.buy().limit_px(price).size(size)
    }

    pub fn limit_sell(self, price: impl ToString, size: impl ToString) -> Self {
        self.sell().limit_px(price).size(size)
    }

    pub fn trigger_buy(
        self,
        trigger_px: impl ToString,
        size: impl ToString,
        tpsl: &str,
    ) -> Self {
        self.buy()
            .size(size)
            .order_type(OrderType::Trigger(Trigger {
                trigger_px: trigger_px.to_string(),
                is_market: true,
                tpsl: tpsl.to_string(),
            }))
    }

    pub fn trigger_sell(
        self,
        trigger_px: impl ToString,
        size: impl ToString,
        tpsl: &str,
    ) -> Self {
        self.sell()
            .size(size)
            .order_type(OrderType::Trigger(Trigger {
                trigger_px: trigger_px.to_string(),
                is_market: true,
                tpsl: tpsl.to_string(),
            }))
    }

    pub fn build(self) -> Result<OrderRequest> {
        let limit_px = self.limit_px.ok_or(HyperliquidError::InvalidRequest(
            "limit_px must be specified".to_string(),
        ))?;
        let sz = self.sz.ok_or(HyperliquidError::InvalidRequest(
            "sz must be specified".to_string(),
        ))?;

        // Parse and format the prices to match API expectations
        let limit_px_f64 = limit_px.parse::<f64>().map_err(|_| {
            HyperliquidError::InvalidRequest("Invalid limit_px format".to_string())
        })?;
        let sz_f64 = sz.parse::<f64>().map_err(|_| {
            HyperliquidError::InvalidRequest("Invalid sz format".to_string())
        })?;

        Ok(OrderRequest {
            asset: self.asset,
            is_buy: self.is_buy.ok_or(HyperliquidError::InvalidRequest(
                "is_buy must be specified".to_string(),
            ))?,
            limit_px: format_float_string(limit_px_f64),
            sz: format_float_string(sz_f64),
            reduce_only: self.reduce_only,
            order_type: self.order_type.unwrap_or(OrderType::Limit(Limit {
                tif: TIF_GTC.to_string(),
            })),
            cloid: self.cloid.map(|id| format!("{:032x}", id.as_u128())),
        })
    }

    pub async fn send(self) -> Result<ExchangeResponseStatus> {
        let provider = self.provider;
        let order = self.build()?;
        provider.place_order(&order).await
    }
}

impl<S: HyperliquidSigner> RawExchangeProvider<S> {
    pub fn order(&self, asset: u32) -> OrderBuilder<'_, S> {
        OrderBuilder::new(self, asset)
    }
}

// ==================== Managed Exchange Provider ====================

use tokio::sync::Mutex as TokioMutex;

use crate::providers::{
    agent::{AgentConfig, AgentManager, AgentWallet},
    batcher::{BatchConfig, OrderBatcher, OrderHandle},
    nonce::NonceManager,
};

/// Configuration for managed exchange provider
#[derive(Clone, Debug)]
pub struct ManagedExchangeConfig {
    /// Enable automatic order batching
    pub batch_orders: bool,
    /// Batch configuration
    pub batch_config: BatchConfig,

    /// Agent lifecycle management
    pub auto_rotate_agents: bool,
    /// Agent configuration
    pub agent_config: AgentConfig,

    /// Nonce isolation per subaccount
    pub isolate_subaccount_nonces: bool,

    /// Safety features
    pub prevent_agent_address_queries: bool,
    pub warn_on_high_nonce_velocity: bool,
}

impl Default for ManagedExchangeConfig {
    fn default() -> Self {
        Self {
            batch_orders: false,
            batch_config: BatchConfig::default(),
            auto_rotate_agents: true,
            agent_config: AgentConfig::default(),
            isolate_subaccount_nonces: true,
            prevent_agent_address_queries: true,
            warn_on_high_nonce_velocity: true,
        }
    }
}

/// Managed exchange provider with safety features and optimizations
pub struct ManagedExchangeProvider<S: HyperliquidSigner> {
    /// Inner raw provider
    inner: Arc<RawExchangeProvider<S>>,

    /// Agent manager for lifecycle
    agent_manager: Option<Arc<AgentManager<S>>>,

    /// Nonce tracking
    nonce_manager: Arc<NonceManager>,

    /// Order batching
    batcher: Option<Arc<OrderBatcher>>,
    batcher_handle: Option<Arc<TokioMutex<Option<tokio::task::JoinHandle<()>>>>>,

    /// Configuration
    config: ManagedExchangeConfig,
}

impl<S: HyperliquidSigner + Clone + 'static> ManagedExchangeProvider<S> {
    /// Create a builder for managed provider
    pub fn builder(signer: S) -> ManagedExchangeProviderBuilder<S> {
        ManagedExchangeProviderBuilder::new(signer)
    }

    /// Create with default configuration for mainnet
    pub async fn mainnet(signer: S) -> Result<Arc<Self>> {
        Self::builder(signer)
            .with_network(Network::Mainnet)
            .build()
            .await
    }

    /// Create with default configuration for testnet
    pub async fn testnet(signer: S) -> Result<Arc<Self>> {
        Self::builder(signer)
            .with_network(Network::Testnet)
            .build()
            .await
    }

    /// Place an order with all managed features
    pub async fn place_order(&self, order: &OrderRequest) -> Result<OrderHandle> {
        // Get nonce based on configuration
        let nonce = if self.config.auto_rotate_agents {
            if let Some(agent_mgr) = &self.agent_manager {
                let agent = agent_mgr.get_or_rotate_agent("default").await?;
                // Use agent's nonce
                agent.next_nonce()
            } else {
                // Fallback to regular nonce
                self.nonce_manager.next_nonce(None)
            }
        } else {
            // Not using agents, use regular nonce
            if self.config.isolate_subaccount_nonces {
                // For subaccounts, we'd need to extract the address from somewhere
                // For now, just use global nonce
                self.nonce_manager.next_nonce(None)
            } else {
                self.nonce_manager.next_nonce(None)
            }
        };

        // Check nonce validity
        if !NonceManager::is_valid_nonce(nonce) {
            return Err(HyperliquidError::InvalidRequest(
                "Generated nonce is outside valid time bounds".to_string(),
            ));
        }

        // For now, we always use the main provider
        // In a full implementation, we'd need to handle agent signing differently
        // This is a limitation of the current design where we can't easily swap signers

        // Batch or direct execution
        if self.config.batch_orders {
            if let Some(batcher) = &self.batcher {
                Ok(batcher.add_order(order.clone(), nonce).await)
            } else {
                // Fallback to direct
                let result = self.inner.place_order(order).await?;
                Ok(OrderHandle::Immediate(Ok(result)))
            }
        } else {
            // Direct execution
            let result = self.inner.place_order(order).await?;
            Ok(OrderHandle::Immediate(Ok(result)))
        }
    }

    /// Place order immediately, bypassing batch
    pub async fn place_order_immediate(
        &self,
        order: &OrderRequest,
    ) -> Result<ExchangeResponseStatus> {
        self.inner.place_order(order).await
    }

    /// Access the raw provider for advanced usage
    pub fn raw(&self) -> &RawExchangeProvider<S> {
        &self.inner
    }

    /// Get current agent status
    pub async fn get_agent_status(&self) -> Option<Vec<(String, AgentWallet)>> {
        if let Some(agent_mgr) = &self.agent_manager {
            Some(agent_mgr.get_active_agents().await)
        } else {
            None
        }
    }

    /// Shutdown the managed provider cleanly
    pub async fn shutdown(self: Arc<Self>) {
        // Stop batcher if running
        if let Some(handle_mutex) = &self.batcher_handle {
            if let Some(handle) = handle_mutex.lock().await.take() {
                handle.abort();
            }
        }
    }
}

/// Builder for ManagedExchangeProvider
pub struct ManagedExchangeProviderBuilder<S: HyperliquidSigner> {
    signer: S,
    network: Network,
    config: ManagedExchangeConfig,
    vault_address: Option<Address>,
    initial_agent: Option<String>,
    builder_address: Option<Address>,
}

impl<S: HyperliquidSigner + Clone + 'static> ManagedExchangeProviderBuilder<S> {
    fn new(signer: S) -> Self {
        Self {
            signer,
            network: Network::Mainnet,
            config: ManagedExchangeConfig::default(),
            vault_address: None,
            initial_agent: None,
            builder_address: None,
        }
    }

    /// Set network
    pub fn with_network(mut self, network: Network) -> Self {
        self.network = network;
        self
    }

    /// Enable automatic order batching
    pub fn with_auto_batching(mut self, interval: std::time::Duration) -> Self {
        self.config.batch_orders = true;
        self.config.batch_config.interval = interval;
        self
    }

    /// Configure agent rotation
    pub fn with_agent_rotation(mut self, ttl: std::time::Duration) -> Self {
        self.config.auto_rotate_agents = true;
        self.config.agent_config.ttl = ttl;
        self
    }

    /// Start with an agent
    pub fn with_agent(mut self, name: Option<String>) -> Self {
        self.initial_agent = name;
        self.config.auto_rotate_agents = true;
        self
    }

    /// Set vault address
    pub fn with_vault(mut self, vault: Address) -> Self {
        self.vault_address = Some(vault);
        self
    }

    /// Set builder address
    pub fn with_builder(mut self, builder: Address) -> Self {
        self.builder_address = Some(builder);
        self
    }

    /// Disable agent rotation
    pub fn without_agent_rotation(mut self) -> Self {
        self.config.auto_rotate_agents = false;
        self
    }

    /// Build the provider
    pub async fn build(self) -> Result<Arc<ManagedExchangeProvider<S>>> {
        // Create raw provider
        let raw = match self.network {
            Network::Mainnet => {
                if let Some(vault) = self.vault_address {
                    RawExchangeProvider::mainnet_vault(self.signer.clone(), vault)
                } else if let Some(builder) = self.builder_address {
                    RawExchangeProvider::mainnet_builder(self.signer.clone(), builder)
                } else {
                    RawExchangeProvider::mainnet(self.signer.clone())
                }
            }
            Network::Testnet => {
                if let Some(vault) = self.vault_address {
                    RawExchangeProvider::testnet_vault(self.signer.clone(), vault)
                } else if let Some(builder) = self.builder_address {
                    RawExchangeProvider::testnet_builder(self.signer.clone(), builder)
                } else {
                    RawExchangeProvider::testnet(self.signer.clone())
                }
            }
        };

        let inner = Arc::new(raw);

        // Create agent manager if needed
        let agent_manager = if self.config.auto_rotate_agents {
            Some(Arc::new(AgentManager::new(
                self.signer,
                self.config.agent_config.clone(),
                self.network,
            )))
        } else {
            None
        };

        // Create nonce manager
        let nonce_manager =
            Arc::new(NonceManager::new(self.config.isolate_subaccount_nonces));

        // Create batcher if needed
        let (batcher, batcher_handle) = if self.config.batch_orders {
            let (batcher, handle) = OrderBatcher::new(self.config.batch_config.clone());
            let batcher = Arc::new(batcher);

            // Spawn batch processing task
            let inner_clone = inner.clone();
            let inner_clone2 = inner.clone();
            let handle_future = tokio::spawn(async move {
                handle
                    .run(
                        move |orders| {
                            let inner = inner_clone.clone();
                            Box::pin(async move {
                                // Execute batch
                                let order_requests: Vec<OrderRequest> =
                                    orders.iter().map(|o| o.order.clone()).collect();

                                match inner.bulk_orders(order_requests).await {
                                    Ok(status) => {
                                        // Return same status for all orders in batch
                                        orders
                                            .iter()
                                            .map(|_| Ok(status.clone()))
                                            .collect()
                                    }
                                    Err(e) => {
                                        // Return same error for all orders in batch
                                        let err_str = e.to_string();
                                        orders
                                            .iter()
                                            .map(|_| {
                                                Err(HyperliquidError::InvalidResponse(
                                                    err_str.clone(),
                                                ))
                                            })
                                            .collect()
                                    }
                                }
                            })
                        },
                        move |cancels| {
                            let inner = inner_clone2.clone();
                            Box::pin(async move {
                                // Execute cancel batch
                                let cancel_requests: Vec<CancelRequest> =
                                    cancels.iter().map(|c| c.cancel.clone()).collect();

                                match inner.bulk_cancel(cancel_requests).await {
                                    Ok(status) => {
                                        // Return same status for all cancels in batch
                                        cancels
                                            .iter()
                                            .map(|_| Ok(status.clone()))
                                            .collect()
                                    }
                                    Err(e) => {
                                        // Return same error for all cancels in batch
                                        let err_str = e.to_string();
                                        cancels
                                            .iter()
                                            .map(|_| {
                                                Err(HyperliquidError::InvalidResponse(
                                                    err_str.clone(),
                                                ))
                                            })
                                            .collect()
                                    }
                                }
                            })
                        },
                    )
                    .await;
            });

            (
                Some(batcher),
                Some(Arc::new(TokioMutex::new(Some(handle_future)))),
            )
        } else {
            (None, None)
        };

        let provider = Arc::new(ManagedExchangeProvider {
            inner,
            agent_manager,
            nonce_manager,
            batcher,
            batcher_handle,
            config: self.config,
        });

        // Initialize agent if requested
        if let Some(agent_name) = self.initial_agent {
            if let Some(agent_mgr) = &provider.agent_manager {
                agent_mgr.get_or_rotate_agent(&agent_name).await?;
            }
        }

        Ok(provider)
    }
}
