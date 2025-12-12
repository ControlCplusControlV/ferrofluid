use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ==================== Order Types ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    #[serde(rename = "a")]
    pub asset: u32,
    #[serde(rename = "b")]
    pub is_buy: bool,
    #[serde(rename = "p")]
    pub limit_px: String,
    #[serde(rename = "s")]
    pub sz: String,
    #[serde(rename = "r", default)]
    pub reduce_only: bool,
    #[serde(rename = "t")]
    pub order_type: OrderType,
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub cloid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderType {
    Limit(Limit),
    Trigger(Trigger),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Limit {
    pub tif: String, // "Alo", "Ioc", "Gtc"
}

/// Trigger order type for stop-loss and take-profit orders.
///
/// **Important**: Field order matters for MessagePack serialization!
/// The order must match the Hyperliquid Python SDK: isMarket, triggerPx, tpsl
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trigger {
    #[serde(rename = "isMarket")]
    pub is_market: bool,
    #[serde(rename = "triggerPx")]
    pub trigger_px: String,
    pub tpsl: String, // "tp" or "sl"
}

// ==================== Cancel Types ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelRequest {
    #[serde(rename = "a")]
    pub asset: u32,
    #[serde(rename = "o")]
    pub oid: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelRequestCloid {
    pub asset: u32,
    pub cloid: String,
}

// ==================== Modify Types ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyRequest {
    pub oid: u64,
    pub order: OrderRequest,
}

// ==================== Builder Types ====================

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuilderInfo {
    #[serde(rename = "b")]
    pub builder: String,
    #[serde(rename = "f")]
    pub fee: u64,
}

// ==================== Convenience Methods ====================

impl OrderRequest {
    /// Create a limit order
    pub fn limit(
        asset: u32,
        is_buy: bool,
        limit_px: impl Into<String>,
        sz: impl Into<String>,
        tif: impl Into<String>,
    ) -> Self {
        Self {
            asset,
            is_buy,
            limit_px: limit_px.into(),
            sz: sz.into(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit { tif: tif.into() }),
            cloid: None,
        }
    }

    /// Create a trigger order (stop loss or take profit)
    pub fn trigger(
        asset: u32,
        is_buy: bool,
        trigger_px: impl Into<String>,
        sz: impl Into<String>,
        tpsl: impl Into<String>,
        is_market: bool,
    ) -> Self {
        Self {
            asset,
            is_buy,
            limit_px: "0".to_string(), // Triggers don't use limit_px
            sz: sz.into(),
            reduce_only: false,
            order_type: OrderType::Trigger(Trigger {
                is_market,
                trigger_px: trigger_px.into(),
                tpsl: tpsl.into(),
            }),
            cloid: None,
        }
    }

    /// Set client order ID
    pub fn with_cloid(mut self, cloid: Option<Uuid>) -> Self {
        self.cloid = cloid.map(|id| format!("{:032x}", id.as_u128()));
        self
    }

    /// Set reduce only
    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = reduce_only;
        self
    }
}

// Convenience constructors for Cancel types
impl CancelRequest {
    pub fn new(asset: u32, oid: u64) -> Self {
        Self { asset, oid }
    }
}

impl CancelRequestCloid {
    pub fn new(asset: u32, cloid: Uuid) -> Self {
        Self {
            asset,
            cloid: format!("{:032x}", cloid.as_u128()),
        }
    }
}
