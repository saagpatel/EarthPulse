# EarthPulse Common Tasks

## Launch locally

```bash
pnpm preflight
pnpm exec tauri dev
```

## Run browser preview smoke

```bash
pnpm exec playwright install chromium
pnpm test:e2e
```

Run this lane when user-facing UI, map/replay controls, or rendered reports change.
Playwright owns the Vite server on `127.0.0.1:4173`; do not pre-start that server.
The browser uses mocked Tauri data, not the native app or real feed results.
Manual `pnpm dev` checks should use a fresh browser profile and synthetic inputs.

## Run full verification

```bash
bash .codex/scripts/run_verify_commands.sh
```

Run from the repository root on a non-`main` branch matching
`codex/<type>/<slug>`, after the [pinned environment setup](environment-setup.md).
The runner also checks repository Git guards and performance artifacts; it is a
broader contributor lane, not the first fixture smoke or a desktop-health claim.
Do not use `dev:lean` or `clean:*` as verification: those wrappers remove local
artifacts.

## Run targeted checks

```bash
pnpm lint
pnpm typecheck
pnpm exec vitest run src/stores/sourceHealthStore.test.ts  # focused fixture
pnpm test:unit                                           # all frontend unit tests
pnpm build                                              # frontend typecheck/build
cargo fmt --check --manifest-path src-tauri/Cargo.toml
cargo check --manifest-path src-tauri/Cargo.toml --locked
cargo test --manifest-path src-tauri/Cargo.toml --locked  # native tests; no app launch
cargo clippy --manifest-path src-tauri/Cargo.toml --locked --all-targets -- -D warnings
```

## Performance sanity

```bash
pnpm perf:bundle
pnpm perf:assets
```
