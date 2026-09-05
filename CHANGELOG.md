# Changelog

All notable changes to this project are documented in this file.

## [0.7.1] - 2026-09-05

### Added

- `Apis`: added `wss`, `grpc_web`, and `evm_http_jsonrpc` endpoint lists (with `Wss`, `GrpcWeb`, and `EvmRpc` models) to match the current `chain.schema.json`.
- `get_evm_rpc_endpoints` helper for direct EVM JSON-RPC endpoint discovery.
- `get_evm_rpc_endpoints_with_overrides` helper for EVM endpoints merged from a local overrides file.
- `ChainOverrides` now also accepts `wss`, `grpc_web`, and `evm_http_jsonrpc` endpoints for local merging.

## [0.7.0] - 2026-09-05

### Added

- `local` module: load additional endpoints (gRPC, REST, RPC) from a local JSON file and merge them into an existing chain's data. This supports endpoints that cannot be made public in the registry. Overrides only extend existing chains; they never create new ones. See `LocalOverrides`, `get_chain_with_overrides`, `get_grpc_endpoints_with_overrides`, `get_rest_endpoints_with_overrides`, and `get_rpc_endpoints_with_overrides`.
- `ChainInfo`: added `chain_type`, `pre_fork_chain_name`, `bech32_config`, `alternative_slip44s`, `images`, `logo_URIs`, `description`, `snapshots`, `keywords`, and `extra_codecs`.
- `Codebase`: added nested `sdk`, `consensus`, `cosmwasm`, `ibc`, `language`, `tag`, and `genesis` models to match the current `chain.schema.json`.
- `Binaries`: added `windows/arm64`.
- `PersistentPeer`: added optional `provider`.
- `Rpc`, `Rest`, `Grpc`: added optional `archive` flag.
- `Explorer`: added `validator_page`, `proposal_page`, and `block_page`.
- `FeeToken`: added `gas_costs`.
- `AssetList`: added `$schema`.
- `Asset`: added `deprecated`, `extended_description`, `type_asset`, `address`, `traces`, `ibc`, `images`, `keywords`, and `socials`. `type_asset` is now represented (it is a required field in the current schema).
- `DenomUnit`: added `aliases`.
- `IBCPath`: added `operators`; `Chain1`/`Chain2` gained `chain_id`; channel-level `client_id` and `connection_id` are now deserialized.

### Changed

- `ChainInfo` no longer has a top-level `genesis` or `update_link`; genesis moved into `codebase.genesis` per the current schema.
- Removed the obsolete flat `cosmos_sdk_version`, `tendermint_version`, `cosmwasm_version`, and `cosmwasm_enabled` fields from `Codebase`; replaced by the nested `sdk`, `consensus`, `cosmwasm`, and `ibc` models.
- `DenomUnit.exponent` is now `i32` to tolerate negative exponents.

### Fixed

- All `chain.json`, `assetlist.json`, and `_IBC/*.json` files in the current registry now deserialize cleanly.
- `registry_cache` tests updated for the current registry (the registry no longer emits `dex` tags and now uses `ACTIVE`/`INACTIVE` status).
- Cache tag assertions now check any channel rather than only the first channel.

## [0.6.0] - 2026-08-11

Release of the endpoint discovery helpers staged in 0.5.0.

- `get_grpc_endpoints`, `get_rest_endpoints`, and `get_rpc_endpoints` helpers for direct endpoint discovery.
- Modernize dependencies: `reqwest` 0.11 -> 0.12 (rustls), `http` 0.2 -> 1.0.
- Fix integration tests for current registry schema (ordered channels, `Option` fields, `ACTIVE` tag status).

## [0.5.0] - 2026-08-11

- Add endpoint discovery helpers: `get_grpc_endpoints`, `get_rest_endpoints`, and `get_rpc_endpoints`.
- Modernize dependencies: `reqwest` 0.11 -> 0.12 (rustls), `http` 0.2 -> 1.0.
- Fix integration tests for current registry schema (ordered channels, `Option` fields, `ACTIVE` tag status).
- Update README and crate docs for the `cosmos_chain_registry` import path.

## [0.4.0] - 2025-09-24

- Add comprehensive IBC path cache filtering: `get_paths_for_chain`, `get_all_paths`, `get_paths_by_channel`, and `get_paths_by_client`.
- Improve tag filtering to handle optional tags.
- Tolerant deserialization for optional/missing fields.
- Re-export common types (`AssetList`, `ChainInfo`, `IBCPath`, `RegistryCache`) at the crate root.
- Update documentation with features list and usage examples.
- Fix clippy warnings.

## [0.2.0-rc3]

- Remove `deny_unknown_fields` serde directive.

## [0.2.0-rc2]

- Add missing fields to chain model.

## [0.2.0-rc1]

- Improve docs by adding examples on the front page.
- Support commit 350840e766f7574a120760a13eda4c466413308a.