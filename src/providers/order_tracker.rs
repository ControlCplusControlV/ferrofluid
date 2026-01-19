use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use uuid::Uuid;

use crate::types::requests::OrderRequest;
use crate::types::responses::ExchangeResponseStatus;

#[derive(Clone, Debug)]
pub struct TrackedOrder {
    pub cloid: Uuid,
    pub order: OrderRequest,
    pub timestamp: u64,
    pub status: OrderStatus,
    pub response: Option<ExchangeResponseStatus>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum OrderStatus {
    Pending,
    Submitted,
    Failed(String),
}

#[derive(Clone)]
pub struct OrderTracker {
    orders: Arc<RwLock<HashMap<Uuid, TrackedOrder>>>,
}

impl OrderTracker {
    pub fn new() -> Self {
        Self {
            orders: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Track a new order
    pub fn track_order(&self, cloid: Uuid, order: OrderRequest, timestamp: u64) {
        let tracked = TrackedOrder {
            cloid,
            order,
            timestamp,
            status: OrderStatus::Pending,
            response: None,
        };

        let mut orders = self.orders.write().unwrap();
        orders.insert(cloid, tracked);
    }

    /// Update order status after submission
    pub fn update_order_status(
        &self,
        cloid: &Uuid,
        status: OrderStatus,
        response: Option<ExchangeResponseStatus>,
    ) {
        let mut orders = self.orders.write().unwrap();
        if let Some(order) = orders.get_mut(cloid) {
            order.status = status;
            order.response = response;
        }
    }

    /// Get a specific order by CLOID
    pub fn get_order(&self, cloid: &Uuid) -> Option<TrackedOrder> {
        let orders = self.orders.read().unwrap();
        orders.get(cloid).cloned()
    }

    /// Get all tracked orders
    pub fn get_all_orders(&self) -> Vec<TrackedOrder> {
        let orders = self.orders.read().unwrap();
        orders.values().cloned().collect()
    }

    /// Get orders by status
    pub fn get_orders_by_status(&self, status: &OrderStatus) -> Vec<TrackedOrder> {
        let orders = self.orders.read().unwrap();
        orders
            .values()
            .filter(|order| &order.status == status)
            .cloned()
            .collect()
    }

    /// Get pending orders
    pub fn get_pending_orders(&self) -> Vec<TrackedOrder> {
        self.get_orders_by_status(&OrderStatus::Pending)
    }

    /// Get submitted orders
    pub fn get_submitted_orders(&self) -> Vec<TrackedOrder> {
        self.get_orders_by_status(&OrderStatus::Submitted)
    }

    /// Get failed orders
    pub fn get_failed_orders(&self) -> Vec<TrackedOrder> {
        let orders = self.orders.read().unwrap();
        orders
            .values()
            .filter(|order| matches!(order.status, OrderStatus::Failed(_)))
            .cloned()
            .collect()
    }

    /// Clear all tracked orders
    pub fn clear(&self) {
        let mut orders = self.orders.write().unwrap();
        orders.clear();
    }

    /// Get the number of tracked orders
    pub fn len(&self) -> usize {
        let orders = self.orders.read().unwrap();
        orders.len()
    }

    /// Check if tracking is empty
    pub fn is_empty(&self) -> bool {
        let orders = self.orders.read().unwrap();
        orders.is_empty()
    }
}

impl Default for OrderTracker {
    fn default() -> Self {
        Self::new()
    }
}
