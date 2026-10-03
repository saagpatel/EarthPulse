# EarthPulse Quickstart

## 1) Clone and Install

```bash
corepack pnpm install --frozen-lockfile
```

## 2) Validate Environment

```bash
pnpm preflight
```

If preflight fails because the path contains `:`, move or symlink the repository to a path without `:` and retry.

## 3) Run App

```bash
pnpm exec tauri dev
```

## 4) Run Contributor Verification

```bash
bash .codex/scripts/run_verify_commands.sh
```

Follow the working-directory/branch and safety requirements in [common tasks](common-tasks.md) before running the broader contributor lane.

## 5) Common Commands

```bash
pnpm lint
pnpm typecheck
pnpm exec vite build
CARGO_TARGET_DIR=/tmp/earthpulse-cargo-target cargo check --manifest-path src-tauri/Cargo.toml
```

## More Onboarding Docs

- `docs/onboarding/environment-setup.md`
- `docs/onboarding/repo-tour.md`
- `docs/onboarding/common-tasks.md`
- `docs/onboarding/first-7-days-plan.md`
- `docs/architecture/system-overview.md`
