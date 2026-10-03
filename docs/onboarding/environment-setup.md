# EarthPulse Environment Setup

## Supported local setup

Use a local path that does **not** contain `:`. The repo preflight rejects colon-delimited paths because they break local tool resolution and Rust dynamic library loading.

Required tools:

- Node.js 22.22.1+ within 22.x, or 24+
- pnpm 10.30.3 (`package.json` package-manager pin)
- Rust 1.96.0 (`rust-toolchain.toml`, including rustfmt and Clippy)
- Git

The locked contributor tooling requires this Node range (`lint-staged` 17.5.1
needs >=22.22.1; jsdom 29.1.1 excludes Node 23). Current CI and `.nvmrc` still select Node 20;
that toolchain mismatch is unresolved and is not a supported full
contributor setup. The preflight minimum is not a substitute for these engines.

## Install sequence

```bash
corepack pnpm install --frozen-lockfile
pnpm preflight
```

If `pnpm preflight` fails:

1. Fix the reported tool or version mismatch first.
2. If the failure mentions `node_modules`, run `pnpm install`.
3. If the failure mentions the repo path, move or create a worktree in a colon-free directory and retry.

## Optional configuration

Set `EARTHPULSE_NASA_API_KEY` or `NASA_API_KEY` in the native process environment only if NASA demo-key limits affect local smoke testing. The Rust backend does not load `.env` files. A key is not required for first launch.

## Local run modes

- Desktop truth: `pnpm exec tauri dev` (starts live feed polling and uses local SQLite)
- Browser preview with mocked desktop data: `pnpm dev`

Use the browser preview for UI checks in a fresh browser profile; it stores mock
settings in localStorage and may still load external map tiles. It does not prove
live-feed health. Run native development only with a disposable app-data context
when validating changes, not a personal watchlist/history database.

On macOS, native compilation needs Xcode Command Line Tools and the Tauri system
prerequisites. Linux CI installs GTK/WebKit and related libraries in
`.github/workflows/quality-gates.yml`. The pinned toolchain governs native builds;
preflight's older minimum version check is not a replacement for that pin.

For an isolated verification checkout, `corepack pnpm install --frozen-lockfile
--ignore-scripts` avoids the `prepare` hook changing shared Git configuration.
This is sufficient for the frontend fixture checks below; normal developer setup
may install the Husky hooks. See [common tasks](common-tasks.md).
