# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Info API
- `meta_and_asset_ctxs()` - Get perpetual metadata with asset contexts
- `frontend_open_orders(user)` - Get open orders with extra frontend metadata
- `user_fills_by_time(user, start, end, aggregate)` - Get fills within a time range
- `historical_orders(user)` - Get user's historical orders
- `sub_accounts(user)` - Get sub-account information
- `user_rate_limit(user)` - Get API rate limit configuration
- `user_vault_equities(user)` - Get vault equity positions

#### Exchange API
- `schedule_cancel(time)` - Schedule automatic order cancellation
- `create_sub_account(name)` - Create a sub-account
- `sub_account_transfer(sub_account, is_deposit, usd)` - Transfer USD to/from sub-account
- `sub_account_spot_transfer(sub_account, is_deposit, token, amount)` - Transfer spot tokens to/from sub-account
- `usd_class_transfer(amount, to_perp)` - Transfer USD between perp and spot classes

#### WebSocket
- `subscribe_bbo(coin)` - Subscribe to best bid/offer updates
- `subscribe_open_orders(user)` - Subscribe to real-time open orders
- `subscribe_clearinghouse_state(user)` - Subscribe to real-time clearinghouse state

#### Documentation
- Created `docs/API_AUDIT.md` with comprehensive API coverage analysis
- Documented Phase 2 and Phase 3 implementation plans for future work

### Changed
- Added `Clone` derive to `MarginSummary` type

## [0.1.1] - 2024-XX-XX

### Fixed
- Fixed trigger order limit_px field for trigger orders
- Corrected field order in trigger order serialization
