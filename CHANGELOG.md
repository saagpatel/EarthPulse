# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-03-13

### Added
- Initial release: EarthPulse with all features and audit fixes
- Lean dev mode and cleanup scripts
- Hardened quality and unsigned release flow

### Fixed
- Runtime crashes: use tauri::async_runtime::spawn, remove invalid notification plugin config
- ESLint errors: render purity, mixed exports, unused expression
- CI: skip local secret guard without gitleaks
- CI: handle detached HEAD in branch guard
- CI: restrict dependabot bypass to bot actor
- CI: keep release workflow valid on GitHub
- CI: reduce workflow noise and prep release signing
- CI: install tauri linux dependencies in quality jobs
- CI: align quality gate with repo toolchain
- CI: install pnpm before quality gate caching
- Rust: satisfy clippy warnings in tauri backend
- Preview smoke and accessibility hardening

### Changed
- Rewrote project overview in README
- Patched flatted audit vulnerability
- Completed handoff and closeout package
- Finalized codex OS bootstrap baseline
- Bootstrapped codex OS guardrails, tests, and docs defaults
- Pruned project bloat and enforced artifact hygiene
- Removed deprecated husky hook bootstrap
