# Hyperliquid API Audit Report

**Date:** 2025-12-13
**SDK Version:** 0.1.1
**API Reference:** https://hyperliquid.gitbook.io/hyperliquid-docs/

## Overview

This document provides a comprehensive audit of the ferrofluid Rust SDK implementation against the official Hyperliquid API documentation. It identifies gaps, tracks implementation status, and provides guidance for future development.

---

## Implementation Status Summary

| Category | Implemented | Missing | Coverage |
|----------|-------------|---------|----------|
| Exchange API | 15 | 25+ | ~40% |
| Info API | 16 | 21 | ~43% |
| WebSocket | 11 | 9 | ~55% |

---

## Exchange API

### Implemented Actions

| Action | Method | File Location |
|--------|--------|---------------|
| `order` | `place_order()`, `bulk_orders()` | `exchange.rs:263` |
| `cancel` | `cancel_order()`, `bulk_cancel()` | `exchange.rs:410` |
| `cancelByCloid` | `cancel_order_by_cloid()`, `bulk_cancel_by_cloid()` | `exchange.rs:424` |
| `batchModify` | `modify_order()`, `bulk_modify()` | `exchange.rs:438` |
| `updateLeverage` | `update_leverage()` | `exchange.rs:538` |
| `updateIsolatedMargin` | `update_isolated_margin()` | `exchange.rs:552` |
| `setReferrer` | `set_referrer()` | `exchange.rs:566` |
| `usdSend` | `usd_transfer()` | `exchange.rs:573` |
| `withdraw3` | `withdraw()` | `exchange.rs:596` |
| `spotSend` | `spot_transfer()` | `exchange.rs:619` |
| `approveAgent` | `approve_agent()`, `approve_agent_new()` | `exchange.rs:645` |
| `approveBuilderFee` | `approve_builder_fee()` | `exchange.rs:714` |
| `vaultTransfer` | `vault_transfer()` | `exchange.rs:739` |
| `spotUser` | `spot_transfer_to_perp()` | `exchange.rs:756` |

### Phase 1 - Missing (HIGH PRIORITY)

| Action | Description | Parameters | Status |
|--------|-------------|------------|--------|
| `scheduleCancel` | Schedule automatic order cancellation | `time: u64` | TODO |
| `createSubAccount` | Create a sub-account | `name: Option<String>` | TODO |
| `subAccountTransfer` | Transfer USD between sub-accounts | `subAccountUser: Address, isDeposit: bool, usd: u64` | TODO |
| `subAccountSpotTransfer` | Transfer spot tokens between sub-accounts | `subAccountUser: Address, isDeposit: bool, token: String, amount: String` | TODO |
| `usdClassTransfer` | Transfer USD between perp/spot classes | `amount: String, toPerp: bool` | TODO |

### Phase 2 - Missing (MEDIUM PRIORITY)

| Action | Description | Parameters |
|--------|-------------|------------|
| `twapOrder` | Place TWAP order | `orders: Vec<TwapOrderRequest>` |
| `twapCancel` | Cancel TWAP order | `a: u32, t: u64` (asset, twapId) |
| `convertToMultiSigUser` | Enable multi-sig for account | `authorizedUsers: Vec<Address>, threshold: u32` |
| `multiSig` | Execute multi-sig transaction | `signatureChainId, payload, signatures` |
| `agentEnableDexAbstraction` | Enable DEX abstraction for agent | `agentAddress: Address` |

### Phase 3 - Missing (LOW PRIORITY)

#### Spot Deployment Actions
| Action | Description |
|--------|-------------|
| `spotDeployRegisterToken` | Register a new spot token |
| `spotDeployUserGenesis` | User genesis for spot deployment |
| `spotDeployFreezeUser` | Freeze user in spot deployment |
| `spotDeployEnableFreezePrivilege` | Enable freeze privilege |
| `spotDeployRevokeFreezePrivilege` | Revoke freeze privilege |
| `spotDeployEnableQuoteToken` | Enable quote token |
| `spotDeployGenesis` | Genesis for spot deployment |
| `spotDeployRegisterSpot` | Register spot pair |
| `spotDeployRegisterHyperliquidity` | Register hyperliquidity |
| `spotDeploySetDeployerTradingFeeShare` | Set deployer fee share |

#### Perp Deployment Actions
| Action | Description |
|--------|-------------|
| `perpDeployRegisterAsset` | Register a perpetual asset |
| `perpDeploySetOracle` | Set oracle for perp asset |

#### Validator/Staking Actions
| Action | Description |
|--------|-------------|
| `cSignerUnjailSelf` | Unjail signer |
| `cSignerJailSelf` | Jail signer |
| `cValidatorRegister` | Register as validator |
| `cValidatorChangeProfile` | Change validator profile |
| `cValidatorUnregister` | Unregister validator |
| `tokenDelegate` | Delegate tokens to validator |

#### Other
| Action | Description |
|--------|-------------|
| `useBigBlocks` | Enable large block mode |
| `noop` | No-operation action |

---

## Info API

### Implemented Queries

| Query Type | Method | File Location |
|------------|--------|---------------|
| `allMids` | `all_mids()` | `info.rs:141` |
| `clearinghouseState` | `user_state()` | `info.rs:148` |
| `l2Book` | `l2_book()` | `info.rs:159` |
| `orderStatus` | `order_status()` | `info.rs:171` |
| `openOrders` | `open_orders()` | `info.rs:184` |
| `userFills` | `user_fills()` | `info.rs:195` |
| `userFunding` | `user_funding()` | `info.rs:206` |
| `userFees` | `user_fees()` | `info.rs:225` |
| `recentTrades` | `recent_trades()` | `info.rs:236` |
| `spotClearinghouseState` | `user_token_balances()` | `info.rs:248` |
| `referral` | `referral()` | `info.rs:259` |
| `meta` | `meta()` | `info.rs:270` |
| `spotMeta` | `spot_meta()` | `info.rs:277` |
| `spotMetaAndAssetCtxs` | `spot_meta_and_asset_ctxs()` | `info.rs:284` |
| `candleSnapshot` | `candles().send()` | `info.rs:295` |
| `fundingHistory` | `funding_history().send()` | `info.rs:305` |

### Phase 1 - Missing (HIGH PRIORITY)

| Query Type | Description | Parameters | Response Type |
|------------|-------------|------------|---------------|
| `metaAndAssetCtxs` | Perp metadata with asset contexts | None | `MetaAndAssetCtxs` |
| `frontendOpenOrders` | Open orders with extra metadata | `user: Address` | `Vec<FrontendOpenOrder>` |
| `userFillsByTime` | Fills within time range | `user, startTime, endTime?, aggregateByTime?` | `Vec<UserFill>` |
| `historicalOrders` | User's historical orders | `user: Address` | `Vec<HistoricalOrder>` |
| `subAccounts` | Sub-account information | `user: Address` | `Vec<SubAccount>` |
| `userRateLimit` | API rate limit configuration | `user: Address` | `UserRateLimit` |
| `userVaultEquities` | Vault equity positions | `user: Address` | `Vec<VaultEquity>` |

### Phase 2 - Missing (MEDIUM PRIORITY)

| Query Type | Description | Parameters |
|------------|-------------|------------|
| `portfolio` | Comprehensive portfolio performance | `user: Address` |
| `userNonFundingLedgerUpdates` | Ledger activity (deposits, withdrawals) | `user, startTime, endTime?` |
| `extraAgents` | Additional authorized agents | `user: Address` |
| `userRole` | User role and account type | `user: Address` |
| `tokenDetails` | Token information | `tokenId: String` |

### Phase 3 - Missing (LOW PRIORITY)

#### Staking/Delegation
| Query Type | Description |
|------------|-------------|
| `delegatorSummary` | Staking summary |
| `delegations` | Staking delegations |
| `delegatorRewards` | Historic staking rewards |
| `delegatorHistory` | Comprehensive staking history |

#### Deployment
| Query Type | Description |
|------------|-------------|
| `perpDeployAuctionStatus` | Perp deployment auction status |
| `spotDeployState` | Spot deployment auction status |
| `spotPairDeployAuctionStatus` | Spot pair deployment auction status |

#### Other
| Query Type | Description |
|------------|-------------|
| `perpDexs` | Available perpetual DEXs |
| `userDexAbstraction` | DEX abstraction state |
| `userToMultiSigSigners` | Multi-signature signers |
| `userTwapSliceFills` | TWAP execution fills |

---

## WebSocket Subscriptions

### Implemented Subscriptions

| Channel | Subscription Type | File Location |
|---------|-------------------|---------------|
| `allMids` | `Subscription::AllMids` | `ws.rs:12` |
| `notification` | `Subscription::Notification` | `ws.rs:13` |
| `webData2` | `Subscription::WebData2` | `ws.rs:14` |
| `candle` | `Subscription::Candle` | `ws.rs:15` |
| `l2Book` | `Subscription::L2Book` | `ws.rs:16` |
| `trades` | `Subscription::Trades` | `ws.rs:17` |
| `orderUpdates` | `Subscription::OrderUpdates` | `ws.rs:18` |
| `userEvents` | `Subscription::UserEvents` | `ws.rs:19` |
| `userFills` | `Subscription::UserFills` | `ws.rs:20` |
| `userFundings` | `Subscription::UserFundings` | `ws.rs:21` |
| `userNonFundingLedgerUpdates` | `Subscription::UserNonFundingLedgerUpdates` | `ws.rs:22` |

### Phase 1 - Missing (HIGH PRIORITY)

| Channel | Description | Parameters |
|---------|-------------|------------|
| `bbo` | Best bid/offer updates | `coin: String` |
| `openOrders` | Real-time open orders | `user: Address` |
| `clearinghouseState` | Real-time clearinghouse data | `user: Address` |

### Phase 2 - Missing (MEDIUM PRIORITY)

| Channel | Description | Parameters |
|---------|-------------|------------|
| `webData3` | Aggregate user info (newer version) | `user: Address` |
| `twapStates` | TWAP order states | `user: Address` |
| `activeAssetCtx` | Active asset context | `coin: String` |
| `activeAssetData` | Active perp asset data | `coin: String` |
| `userTwapSliceFills` | TWAP slice fills | `user: Address` |
| `userTwapHistory` | TWAP history | `user: Address` |

---

## Implementation Guidelines

### Adding New Info Queries

1. Add response type to `src/types/info_types.rs`
2. Add method to `InfoProvider` in `src/providers/info.rs`
3. Follow existing pattern:

```rust
pub async fn new_query(
    &self,
    user: Address,
) -> Result<NewQueryResponse, HyperliquidError> {
    let request = json!({
        "type": "newQuery",
        "user": user
    });
    self.request(request).await
}
```

### Adding New Exchange Actions

1. Add action struct to `src/types/actions.rs` if needed
2. Add method to `RawExchangeProvider` in `src/providers/exchange.rs`
3. For L1 actions (most trading actions):

```rust
pub async fn new_action(
    &self,
    param: Type,
) -> Result<ExchangeResponseStatus> {
    let action = NewAction { param };
    self.send_l1_action("newAction", &action).await
}
```

4. For User actions (require EIP-712 signing):

```rust
pub async fn new_user_action(
    &self,
    param: Type,
) -> Result<ExchangeResponseStatus> {
    let action = NewUserAction {
        signature_chain_id: chain_id,
        // ... other fields
    };
    self.send_user_action(&action).await
}
```

### Adding New WebSocket Subscriptions

1. Add variant to `Subscription` enum in `src/types/ws.rs`
2. Add message type to `Message` enum
3. Add convenience method to `RawWsProvider`:

```rust
pub async fn subscribe_new_channel(
    &mut self,
    param: Type,
) -> Result<(SubscriptionId, UnboundedReceiver<Message>), HyperliquidError> {
    let subscription = Subscription::NewChannel { param };
    self.subscribe(subscription).await
}
```

---

## Testing Guidelines

- All new endpoints should have integration tests in `tests/`
- Use testnet for testing trading actions
- Mock responses for Info queries when possible
- Test error handling for invalid parameters

---

## Changelog

### Phase 1 (Current Implementation)
- [ ] Added `metaAndAssetCtxs` Info query
- [ ] Added `frontendOpenOrders` Info query
- [ ] Added `userFillsByTime` Info query
- [ ] Added `historicalOrders` Info query
- [ ] Added `subAccounts` Info query
- [ ] Added `userRateLimit` Info query
- [ ] Added `userVaultEquities` Info query
- [ ] Added `scheduleCancel` Exchange action
- [ ] Added `createSubAccount` Exchange action
- [ ] Added `subAccountTransfer` Exchange action
- [ ] Added `subAccountSpotTransfer` Exchange action
- [ ] Added `usdClassTransfer` Exchange action
- [ ] Added `bbo` WebSocket subscription
- [ ] Added `openOrders` WebSocket subscription
- [ ] Added `clearinghouseState` WebSocket subscription

### Phase 2 (Future)
- See sections marked as Phase 2 above

### Phase 3 (Future)
- See sections marked as Phase 3 above

---

## References

- [Hyperliquid API Docs](https://hyperliquid.gitbook.io/hyperliquid-docs/)
- [Exchange Endpoint](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/exchange-endpoint)
- [Info Endpoint](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/info-endpoint)
- [WebSocket](https://hyperliquid.gitbook.io/hyperliquid-docs/for-developers/api/websocket)
- [Python SDK Reference](https://github.com/hyperliquid-dex/hyperliquid-python-sdk)
