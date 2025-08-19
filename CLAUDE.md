# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library for interacting with the Cosmos chain registry repository. It provides models and APIs for deserializing chain.json, assets.json, and IBC path JSON files from the Cosmos chain registry.

## Common Development Commands

### Build & Test
- Build the library: `cargo build`
- Build with all features: `cargo build --all-features`
- Run tests: `cargo test`
- Run tests with cache feature: `cargo test --all-features`
- Run a specific test: `cargo test registry_cache_happy_path`
- Check code without building: `cargo check`
- Build documentation: `cargo doc --open`

### Code Quality
- Format code: `cargo fmt`
- Run clippy linter: `cargo clippy`
- Fix linting issues automatically: `cargo fix`

### Publishing
- Check package before publishing: `cargo publish --dry-run`
- Build docs with all features (for docs.rs): `cargo doc --all-features`

## Architecture & Key Components

### Core Modules (src/)
- **lib.rs**: Main entry point exposing public API and examples
- **get.rs**: Functions for fetching data from the chain registry GitHub repository
  - Uses a fixed git ref: `350840e766f7574a120760a13eda4c466413308a`
  - Provides `get_chain()`, `get_assets()`, `get_path()`, `list_chains()`, `list_paths()`
- **chain.rs**: Models for deserializing chain.json files
- **assets.rs**: Models for deserializing assets.json files  
- **paths.rs**: Models for IBC path JSON serialization/deserialization
- **cache.rs**: RegistryCache type for loading and filtering IBC paths (feature-gated)
- **github.rs**: GitHub API interaction models

### Features
- `default = ["cache"]`: Includes caching functionality
- `cache`: Enables the RegistryCache type for filtering IBC paths

### Testing
Tests use the `assay` test framework. Integration tests are in `tests/registry_cache.rs` and require the cache feature.

## Important Context

1. The chain registry is unversioned and syntax is unenforced - the library ignores unrecognized JSON fields
2. The library currently uses a pinned git reference to ensure consistency
3. RegistryCache construction requires individual GET requests for every path (can be slow)
4. The cache currently only supports IBC Path data, not chain/asset info
5. Documentation is built with `--all-features` flag for docs.rs