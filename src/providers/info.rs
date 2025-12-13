use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

use alloy::primitives::Address;
use http::{Method, Request};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::{connect::HttpConnector, Client};
use hyper_util::rt::TokioExecutor;
use serde_json::json;

use crate::constants::Network;
use crate::errors::HyperliquidError;
use crate::types::info_types::*;
use crate::types::Symbol;

// Rate limiter implementation
pub struct RateLimiter {
    tokens: Mutex<f64>,
    max_tokens: f64,
    refill_rate: f64,
    last_refill: Mutex<Instant>,
}

impl RateLimiter {
    pub fn new(max_tokens: u32, refill_rate: u32) -> Self {
        Self {
            tokens: Mutex::new(max_tokens as f64),
            max_tokens: max_tokens as f64,
            refill_rate: refill_rate as f64,
            last_refill: Mutex::new(Instant::now()),
        }
    }

    pub fn check_weight(&self, weight: u32) -> Result<(), HyperliquidError> {
        let mut tokens = self.tokens.lock().unwrap();
        let mut last_refill = self.last_refill.lock().unwrap();

        // Refill tokens based on elapsed time
        let now = Instant::now();
        let elapsed = now.duration_since(*last_refill).as_secs_f64();
        let tokens_to_add = elapsed * self.refill_rate;

        *tokens = (*tokens + tokens_to_add).min(self.max_tokens);
        *last_refill = now;

        // Check if we have enough tokens
        if *tokens >= weight as f64 {
            *tokens -= weight as f64;
            Ok(())
        } else {
            Err(HyperliquidError::RateLimited {
                available: *tokens as u32,
                required: weight,
            })
        }
    }
}

pub struct InfoProvider {
    client: Client<HttpsConnector<HttpConnector>, Full<Bytes>>,
    endpoint: &'static str,
}

impl InfoProvider {
    pub fn mainnet() -> Self {
        Self::new(Network::Mainnet)
    }

    pub fn testnet() -> Self {
        Self::new(Network::Testnet)
    }

    pub fn new(network: Network) -> Self {
        // Initialize rustls crypto provider if not already set
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .expect("TLS initialization failed")
            .https_only()
            .enable_http1()
            .build();

        let client = Client::builder(TokioExecutor::new()).build(https);

        Self {
            client,
            endpoint: match network {
                Network::Mainnet => "https://api.hyperliquid.xyz/info",
                Network::Testnet => "https://api.hyperliquid-testnet.xyz/info",
            },
        }
    }

    async fn request<T>(
        &self,
        request_json: serde_json::Value,
    ) -> Result<T, HyperliquidError>
    where
        T: serde::de::DeserializeOwned,
    {
        let body_string = serde_json::to_string(&request_json)?;
        let body_bytes = Bytes::from(body_string);

        let req = Request::builder()
            .method(Method::POST)
            .uri(self.endpoint)
            .header("Content-Type", "application/json")
            .body(Full::new(body_bytes))?;

        let res = self
            .client
            .request(req)
            .await
            .map_err(|e| HyperliquidError::Network(e.to_string()))?;
        let status = res.status();

        let body_bytes = res
            .collect()
            .await
            .map_err(|e| HyperliquidError::Network(e.to_string()))?
            .to_bytes();
        let body_str = String::from_utf8_lossy(&body_bytes);

        if !status.is_success() {
            return Err(HyperliquidError::Http {
                status: status.as_u16(),
                body: body_str.to_string(),
            });
        }

        let mut body_vec = body_bytes.to_vec();
        simd_json::from_slice(&mut body_vec).map_err(|e| e.into())
    }

    // ==================== Simple Direct Methods ====================

    pub async fn all_mids(&self) -> Result<HashMap<String, String>, HyperliquidError> {
        let request = json!({
            "type": "allMids"
        });
        self.request(request).await
    }

    pub async fn user_state(
        &self,
        user: Address,
    ) -> Result<UserStateResponse, HyperliquidError> {
        let request = json!({
            "type": "clearinghouseState",
            "user": user
        });
        self.request(request).await
    }

    pub async fn l2_book(
        &self,
        coin: impl Into<Symbol>,
    ) -> Result<L2SnapshotResponse, HyperliquidError> {
        let symbol = coin.into();
        let request = json!({
            "type": "l2Book",
            "coin": symbol.as_str()
        });
        self.request(request).await
    }

    pub async fn order_status(
        &self,
        user: Address,
        oid: u64,
    ) -> Result<OrderStatusResponse, HyperliquidError> {
        let request = json!({
            "type": "orderStatus",
            "user": user,
            "oid": oid
        });
        self.request(request).await
    }

    pub async fn open_orders(
        &self,
        user: Address,
    ) -> Result<Vec<OpenOrdersResponse>, HyperliquidError> {
        let request = json!({
            "type": "openOrders",
            "user": user
        });
        self.request(request).await
    }

    pub async fn user_fills(
        &self,
        user: Address,
    ) -> Result<Vec<UserFillsResponse>, HyperliquidError> {
        let request = json!({
            "type": "userFills",
            "user": user
        });
        self.request(request).await
    }

    pub async fn user_funding(
        &self,
        user: Address,
        start_time: u64,
        end_time: Option<u64>,
    ) -> Result<Vec<UserFundingResponse>, HyperliquidError> {
        let mut request = json!({
            "type": "userFunding",
            "user": user,
            "startTime": start_time
        });

        if let Some(end) = end_time {
            request["endTime"] = json!(end);
        }

        self.request(request).await
    }

    pub async fn user_fees(
        &self,
        user: Address,
    ) -> Result<UserFeesResponse, HyperliquidError> {
        let request = json!({
            "type": "userFees",
            "user": user
        });
        self.request(request).await
    }

    pub async fn recent_trades(
        &self,
        coin: impl Into<Symbol>,
    ) -> Result<Vec<RecentTradesResponse>, HyperliquidError> {
        let symbol = coin.into();
        let request = json!({
            "type": "recentTrades",
            "coin": symbol.as_str()
        });
        self.request(request).await
    }

    pub async fn user_token_balances(
        &self,
        user: Address,
    ) -> Result<UserTokenBalanceResponse, HyperliquidError> {
        let request = json!({
            "type": "spotClearinghouseState",
            "user": user
        });
        self.request(request).await
    }

    pub async fn referral(
        &self,
        user: Address,
    ) -> Result<ReferralResponse, HyperliquidError> {
        let request = json!({
            "type": "referral",
            "user": user
        });
        self.request(request).await
    }

    pub async fn meta(&self) -> Result<Meta, HyperliquidError> {
        let request = json!({
            "type": "meta"
        });
        self.request(request).await
    }

    pub async fn spot_meta(&self) -> Result<SpotMeta, HyperliquidError> {
        let request = json!({
            "type": "spotMeta"
        });
        self.request(request).await
    }

    pub async fn spot_meta_and_asset_ctxs(
        &self,
    ) -> Result<SpotMetaAndAssetCtxs, HyperliquidError> {
        let request = json!({
            "type": "spotMetaAndAssetCtxs"
        });
        self.request(request).await
    }

    // ==================== Builder Pattern Methods ====================

    pub fn candles(&self, coin: impl Into<Symbol>) -> CandlesRequestBuilder {
        CandlesRequestBuilder {
            provider: self,
            coin: coin.into(),
            interval: None,
            start_time: None,
            end_time: None,
        }
    }

    pub fn funding_history(&self, coin: impl Into<Symbol>) -> FundingHistoryBuilder {
        FundingHistoryBuilder {
            provider: self,
            coin: coin.into(),
            start_time: None,
            end_time: None,
        }
    }
}

// ==================== Request Builders ====================

pub struct CandlesRequestBuilder<'a> {
    provider: &'a InfoProvider,
    coin: Symbol,
    interval: Option<String>,
    start_time: Option<u64>,
    end_time: Option<u64>,
}

impl<'a> CandlesRequestBuilder<'a> {
    pub fn interval(mut self, interval: impl Into<String>) -> Self {
        self.interval = Some(interval.into());
        self
    }

    pub fn time_range(mut self, start: u64, end: u64) -> Self {
        self.start_time = Some(start);
        self.end_time = Some(end);
        self
    }

    pub fn start_time(mut self, start: u64) -> Self {
        self.start_time = Some(start);
        self
    }

    pub fn end_time(mut self, end: u64) -> Self {
        self.end_time = Some(end);
        self
    }

    pub async fn send(self) -> Result<Vec<CandlesSnapshotResponse>, HyperliquidError> {
        let interval = self.interval.ok_or_else(|| {
            HyperliquidError::InvalidRequest("interval is required".into())
        })?;
        let start_time = self.start_time.ok_or_else(|| {
            HyperliquidError::InvalidRequest("start_time is required".into())
        })?;
        let end_time = self.end_time.ok_or_else(|| {
            HyperliquidError::InvalidRequest("end_time is required".into())
        })?;

        let request = json!({
            "type": "candleSnapshot",
            "req": {
                "coin": self.coin.as_str(),
                "interval": interval,
                "startTime": start_time,
                "endTime": end_time
            }
        });

        self.provider.request(request).await
    }
}

pub struct FundingHistoryBuilder<'a> {
    provider: &'a InfoProvider,
    coin: Symbol,
    start_time: Option<u64>,
    end_time: Option<u64>,
}

impl<'a> FundingHistoryBuilder<'a> {
    pub fn time_range(mut self, start: u64, end: u64) -> Self {
        self.start_time = Some(start);
        self.end_time = Some(end);
        self
    }

    pub fn start_time(mut self, start: u64) -> Self {
        self.start_time = Some(start);
        self
    }

    pub fn end_time(mut self, end: u64) -> Self {
        self.end_time = Some(end);
        self
    }

    pub async fn send(self) -> Result<Vec<FundingHistoryResponse>, HyperliquidError> {
        let start_time = self.start_time.ok_or_else(|| {
            HyperliquidError::InvalidRequest("start_time is required".into())
        })?;

        let mut request = json!({
            "type": "fundingHistory",
            "coin": self.coin.as_str(),
            "startTime": start_time
        });

        if let Some(end) = self.end_time {
            request["endTime"] = json!(end);
        }

        self.provider.request(request).await
    }
}
