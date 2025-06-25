//! Order batching for high-frequency trading strategies

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{mpsc, Mutex};
use tokio::time::interval;
use uuid::Uuid;

use crate::errors::HyperliquidError;
use crate::types::requests::{CancelRequest, OrderRequest};
use crate::types::responses::ExchangeResponseStatus;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

/// Order with metadata for batching
#[derive(Clone)]
pub struct PendingOrder {
    pub order: OrderRequest,
    pub nonce: u64,
    pub id: Uuid,
    pub response_tx:
        mpsc::UnboundedSender<Result<ExchangeResponseStatus, HyperliquidError>>,
}

/// Cancel with metadata for batching
#[derive(Clone)]
pub struct PendingCancel {
    pub cancel: CancelRequest,
    pub nonce: u64,
    pub id: Uuid,
    pub response_tx:
        mpsc::UnboundedSender<Result<ExchangeResponseStatus, HyperliquidError>>,
}

/// Order type classification for priority batching
#[derive(Debug, Clone, PartialEq)]
pub enum OrderPriority {
    /// Add Liquidity Only orders (highest priority per docs)
    ALO,
    /// Regular GTC/IOC orders
    Regular,
}

/// Handle returned when submitting to batcher
pub enum OrderHandle {
    /// Order submitted to batch, will be sent soon
    Pending {
        id: Uuid,
        rx: mpsc::UnboundedReceiver<Result<ExchangeResponseStatus, HyperliquidError>>,
    },
    /// Order executed immediately (when batching disabled)
    Immediate(Result<ExchangeResponseStatus, HyperliquidError>),
}

/// Configuration for order batching
#[derive(Clone, Debug)]
pub struct BatchConfig {
    /// Interval between batch submissions
    pub interval: Duration,
    /// Maximum orders per batch
    pub max_batch_size: usize,
    /// Separate ALO orders into priority batches
    pub prioritize_alo: bool,
    /// Maximum time an order can wait in queue
    pub max_wait_time: Duration,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_millis(100), // 0.1s as recommended
            max_batch_size: 100,
            prioritize_alo: true,
            max_wait_time: Duration::from_millis(500),
        }
    }
}

/// Batches orders for efficient submission
pub struct OrderBatcher {
    /// Pending orders queue
    pending_orders: Arc<Mutex<Vec<PendingOrder>>>,
    /// Pending cancels queue
    pending_cancels: Arc<Mutex<Vec<PendingCancel>>>,
    /// Configuration
    _config: BatchConfig,
    /// Shutdown signal
    shutdown_tx: mpsc::Sender<()>,
}

impl OrderBatcher {
    /// Create a new order batcher
    pub fn new(config: BatchConfig) -> (Self, BatcherHandle) {
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

        let batcher = Self {
            pending_orders: Arc::new(Mutex::new(Vec::new())),
            pending_cancels: Arc::new(Mutex::new(Vec::new())),
            _config: config,
            shutdown_tx,
        };

        let handle = BatcherHandle {
            pending_orders: batcher.pending_orders.clone(),
            pending_cancels: batcher.pending_cancels.clone(),
            shutdown_rx,
        };

        (batcher, handle)
    }

    /// Add an order to the batch queue
    pub async fn add_order(&self, order: OrderRequest, nonce: u64) -> OrderHandle {
        let id = Uuid::new_v4();
        let (tx, rx) = mpsc::unbounded_channel();

        let pending = PendingOrder {
            order,
            nonce,
            id,
            response_tx: tx,
        };

        self.pending_orders.lock().await.push(pending);

        OrderHandle::Pending { id, rx }
    }

    /// Add a cancel to the batch queue
    pub async fn add_cancel(&self, cancel: CancelRequest, nonce: u64) -> OrderHandle {
        let id = Uuid::new_v4();
        let (tx, rx) = mpsc::unbounded_channel();

        let pending = PendingCancel {
            cancel,
            nonce,
            id,
            response_tx: tx,
        };

        self.pending_cancels.lock().await.push(pending);

        OrderHandle::Pending { id, rx }
    }

    /// Shutdown the batcher
    pub async fn shutdown(self) {
        let _ = self.shutdown_tx.send(()).await;
    }
}

/// Handle for the background batching task
pub struct BatcherHandle {
    pending_orders: Arc<Mutex<Vec<PendingOrder>>>,
    pending_cancels: Arc<Mutex<Vec<PendingCancel>>>,
    shutdown_rx: mpsc::Receiver<()>,
}

impl BatcherHandle {
    /// Run the batching loop (should be spawned as a task)
    pub async fn run<F, G>(mut self, mut order_executor: F, mut cancel_executor: G)
    where
        F: FnMut(
                Vec<PendingOrder>,
            )
                -> BoxFuture<Vec<Result<ExchangeResponseStatus, HyperliquidError>>>
            + Send,
        G: FnMut(
                Vec<PendingCancel>,
            )
                -> BoxFuture<Vec<Result<ExchangeResponseStatus, HyperliquidError>>>
            + Send,
    {
        let mut interval = interval(Duration::from_millis(100)); // Fixed interval for now

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    // Process orders
                    let orders = {
                        let mut pending = self.pending_orders.lock().await;
                        std::mem::take(&mut *pending)
                    };

                    if !orders.is_empty() {
                        // Separate ALO from regular orders
                        let (alo_orders, regular_orders): (Vec<_>, Vec<_>) =
                            orders.into_iter().partition(|o| {
                                o.order.is_alo()
                            });

                        // Process ALO orders first (priority)
                        if !alo_orders.is_empty() {
                            let results = order_executor(alo_orders.clone()).await;
                            for (order, result) in alo_orders.into_iter().zip(results) {
                                let _ = order.response_tx.send(result);
                            }
                        }

                        // Process regular orders
                        if !regular_orders.is_empty() {
                            let results = order_executor(regular_orders.clone()).await;
                            for (order, result) in regular_orders.into_iter().zip(results) {
                                let _ = order.response_tx.send(result);
                            }
                        }
                    }

                    // Process cancels
                    let cancels = {
                        let mut pending = self.pending_cancels.lock().await;
                        std::mem::take(&mut *pending)
                    };

                    if !cancels.is_empty() {
                        let results = cancel_executor(cancels.clone()).await;
                        for (cancel, result) in cancels.into_iter().zip(results) {
                            let _ = cancel.response_tx.send(result);
                        }
                    }
                }

                _ = self.shutdown_rx.recv() => {
                    // Graceful shutdown
                    break;
                }
            }
        }
    }
}

impl OrderRequest {
    /// Check if this is an ALO order
    pub fn is_alo(&self) -> bool {
        match &self.order_type {
            crate::types::requests::OrderType::Limit(limit) => {
                limit.tif.to_lowercase() == "alo"
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::requests::{Limit, OrderType};

    #[tokio::test]
    async fn test_order_batching() {
        let config = BatchConfig::default();
        let (batcher, _handle) = OrderBatcher::new(config);

        // Create a test order
        let order = OrderRequest {
            asset: 0,
            is_buy: true,
            limit_px: "50000".to_string(),
            sz: "0.1".to_string(),
            reduce_only: false,
            order_type: OrderType::Limit(Limit {
                tif: "Gtc".to_string(),
            }),
            cloid: None,
        };

        // Add to batch
        let handle = batcher.add_order(order, 123456789).await;

        // Should return pending handle
        assert!(matches!(handle, OrderHandle::Pending { .. }));
    }
}
