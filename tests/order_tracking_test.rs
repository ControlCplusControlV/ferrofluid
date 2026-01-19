#[cfg(test)]
mod tests {
    use alloy::signers::local::PrivateKeySigner;
    use ferrofluid::{
        constants::TIF_GTC, signers::AlloySigner, types::requests::OrderRequest,
        ExchangeProvider,
    };
    use std::sync::Once;
    use uuid::Uuid;

    static INIT: Once = Once::new();

    fn init_crypto() {
        INIT.call_once(|| {
            rustls::crypto::CryptoProvider::install_default(
                rustls::crypto::aws_lc_rs::default_provider(),
            )
            .expect("Failed to install rustls crypto provider");
        });
    }

    fn create_test_exchange() -> ExchangeProvider<AlloySigner<PrivateKeySigner>> {
        init_crypto();
        let private_key =
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let signer = private_key.parse::<PrivateKeySigner>().unwrap();
        let alloy_signer = AlloySigner { inner: signer };

        ExchangeProvider::testnet(alloy_signer).with_order_tracking()
    }

    #[test]
    fn test_order_tracking_initialization() {
        let exchange = create_test_exchange();
        assert_eq!(exchange.tracked_order_count(), 0);
        assert!(exchange.get_all_tracked_orders().is_empty());
    }

    #[test]
    fn test_order_tracking_methods() {
        let exchange = create_test_exchange();

        // Test empty state
        assert_eq!(exchange.get_all_tracked_orders().len(), 0);
        assert_eq!(exchange.get_pending_orders().len(), 0);
        assert_eq!(exchange.get_submitted_orders().len(), 0);
        assert_eq!(exchange.get_failed_orders().len(), 0);

        // Test get by non-existent cloid
        let fake_cloid = Uuid::new_v4();
        assert!(exchange.get_tracked_order(&fake_cloid).is_none());
    }

    #[test]
    fn test_clear_tracked_orders() {
        let exchange = create_test_exchange();

        // Clear should work even when empty
        exchange.clear_tracked_orders();
        assert_eq!(exchange.tracked_order_count(), 0);
    }

    #[test]
    fn test_order_builder_with_tracking() {
        let exchange = create_test_exchange();

        // Create order using builder
        let _order_builder = exchange
            .order(0)
            .buy()
            .limit_px("45000.0")
            .size("0.01")
            .cloid(Uuid::new_v4());

        // Builder doesn't automatically track until order is placed
        assert_eq!(exchange.tracked_order_count(), 0);
    }

    #[test]
    fn test_tracking_disabled_by_default() {
        init_crypto();
        let private_key =
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let signer = private_key.parse::<PrivateKeySigner>().unwrap();
        let alloy_signer = AlloySigner { inner: signer };

        // Create exchange without tracking
        let exchange = ExchangeProvider::testnet(alloy_signer);

        // Methods should return empty results
        assert_eq!(exchange.tracked_order_count(), 0);
        assert_eq!(exchange.get_all_tracked_orders().len(), 0);
        assert_eq!(exchange.get_pending_orders().len(), 0);

        // Clear should be safe to call
        exchange.clear_tracked_orders();
    }

    #[tokio::test]
    async fn test_order_tracking_with_mock_placement() {
        let exchange = create_test_exchange();

        // Create a test order
        let _order = OrderRequest::limit(0, true, "45000.0", "0.01", TIF_GTC);

        // Before placing, no orders tracked
        assert_eq!(exchange.tracked_order_count(), 0);

        // Note: Actually placing the order would require a valid connection
        // This test verifies the tracking infrastructure is in place
    }
}
