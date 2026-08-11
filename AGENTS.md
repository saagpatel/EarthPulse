<!-- comm-contract:start -->

## Communication Contract

- Inherit global Codex communication and reporting rules from `~/.codex/AGENTS.override.md` and `~/.codex/policies/communication/BigPictureReportingV1.md`.
- Repo-specific instructions below add project constraints only; do not restate global voice or status-reporting rules here.
<!-- comm-contract:end -->

## Inherited Operating Rules

- Inherit global git, review/fix, testing, docs, skill-use, and reporting gates from `~/.codex/AGENTS.md` and active session instructions.
- Use `.codex/verify.commands` and `.codex/scripts/run_verify_commands.sh` as this repo-local verification authority when present.
- Keep the project-specific portfolio constraints below as the source of truth for runtime, privacy, and release risks.

<!-- portfolio-context:start -->

# Portfolio Context

## What This Project Is

EarthPulse is a Tauri desktop app that turns live planetary activity into a layered map. It aggregates earthquakes, satellite tracks, aurora/space weather, NASA hazard feeds, asteroid close approaches, and other data streams for filtering, replay, watchlists, stats, and export.

## Current State

The repo is active product work. Existing local changes are PR-template metadata, so context recovery should stay documentation-only.

## Stack

| Layer          | Technology                   |
| -------------- | ---------------------------- |
| Desktop shell  | Tauri 2 + Rust               |
| Frontend       | React 19 + TypeScript + Vite |
| Map rendering  | Leaflet / MapLibre           |
| Satellite math | SGP4 propagator (Rust)       |
| Storage        | SQLite (watchlists, history) |
| Styling        | Tailwind CSS                 |

## How To Run

- Install dependencies with `pnpm install`.
- Start local development with `pnpm dev`.
- Build the desktop app with `pnpm build`.
- Run source-specific and map/replay checks before calling live-data behavior healthy.

## Known Risks

- External data feeds have different refresh intervals and availability; handle stale or missing source data explicitly.
- SGP4 satellite propagation lives in Rust; verify orbital math after backend changes.
- Watchlists and replay history persist in SQLite; avoid destructive local data changes unless explicitly requested.
- Keep PR-template drift separate from live-data or map-rendering changes.

## Next Recommended Move

Resolve the PR-template drift separately, then verify feed polling, map layers, replay, watchlists, and export paths before changing runtime behavior.

<!-- portfolio-context:end -->
