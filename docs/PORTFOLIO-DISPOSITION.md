# EarthPulse — Portfolio Disposition

**Status:** Release Frozen — Tauri 2 + Rust real-time Earth events
visualizer on `origin/main`. **Already self-closed-out** by the
operator: `docs/comms/closeout/final-closeout-report.md` on canonical
main explicitly declares "readiness stabilization pass for internal
beta use" complete. Joins the signing cluster as the 16th member.

> Disposition uses strict `origin/main` verification.
> **First session repo with pre-shipped closeout docs.**

---

## Verification posture

This repo has both `origin` (`saagpatel/EarthPulse`) and
`legacy-origin` (`saagar210/EarthPulse`) remotes. **Local clone's
`main` is tracking `origin/main` correctly** — no trap here.

Specifically verified on `origin/main`:

- Tip: `7ad1c8d` (HEAD)
- **Already-shipped release docs on canonical main:**
  - `docs/comms/closeout/final-closeout-report.md` (the operator's
    own closeout)
  - `docs/comms/closeout/handoff-checklist.md`
  - `docs/comms/closeout/stakeholder-update-package.md`
  - `docs/comms/closeout/workstream-plan.md`
- **Architecture docs on canonical main:**
  - `docs/architecture/data-flow.md`
  - `docs/architecture/dependencies-and-external-services.md`
  - `docs/architecture/frontend-backend-boundary.md`
  - `docs/architecture/system-overview.md`
- Release workflows on `origin/main`:
  - `.github/workflows/release-matrix.yml`
  - `.github/workflows/release-promote.yml`
  - `.github/workflows/quality-foundation.yml`
  - `.github/workflows/quality-gates.yml`
  - `.github/workflows/security-quality.yml`
  - `.github/workflows/artifact-hygiene.yml`
- Tree on `origin/main` is a real Tauri 2 + Rust desktop app:
  - `src-tauri/src/commands/{air_quality,asteroid,earthquake,eonet,gdacs,historical,iss,meteor,plate,replay}.rs`
  - `src-tauri/src/calculations/{orbit,terminator,mod}.rs`
  - `src-tauri/plates.geojson` (tectonic plate boundaries data)
- Default branch: `main`

---

## Legacy-origin orphan note

`legacy-origin/main` has **zero commits** not on `origin/main`. Clean
state.

---

## Current state in one paragraph

EarthPulse is a Tauri 2 + Rust desktop app that visualizes real-time
Earth events. Backend command modules cover air quality (likely
OpenAQ or PurpleAir), asteroid tracking (NeoWs), earthquake feeds,
NASA EONET, GDACS (global disaster alerts), historical event replay,
ISS position, meteor showers, tectonic plate data, and a replay
system. Calculations module covers orbit math and the day/night
terminator. The operator has already shipped a full closeout doc
pack on canonical main describing "readiness stabilization for
internal beta use" as complete — onboarding, architecture, ops,
release flow, and handoff are all documented.

For full detail see:

- `README.md` on `origin/main`
- `docs/comms/closeout/final-closeout-report.md`
- `docs/architecture/system-overview.md`

---

## Why "Release Frozen" instead of other dispositions

- **Active** — wrong. The operator wrote a `final-closeout-report.md`
  on canonical main saying the readiness stabilization is done. That
  is operator-declared closure.
- **Cold Storage / Archived** — wrong. The closeout explicitly
  positions this for "internal beta use," not for archival.
- **Release Frozen** — correct, with a twist: most cluster members
  are gated _only_ on Apple signing. EarthPulse is gated on signing
  - a separate internal-beta-vs-public-release decision. The
    `release-matrix.yml` and `release-promote.yml` workflows suggest
    the operator has a staged release pipeline in mind.

This is the **16th signing cluster member**: DesktopPEt / ContentEngine
/ AIGCCore / Relay / FreeLanceInvoice / Nexus / DeepTank / OPscinema /
ShipKit / SignalFlow / PixelForge / DatabaseSchema / LegalDocsReview /
WorkdayDebrief / TicketDashboard / **EarthPulse**.

---

## Unblock trigger (operator)

When ready to ship:

1. Wire Apple Developer ID + notarization credentials.
2. **Decide internal-beta vs public-release scope.** The closeout
   report calls it "internal beta use"; the release-promote
   workflow suggests a promotion path exists. Operator picks: stay
   internal-beta, or promote to public v1.0.
3. **Confirm external API key posture.** Five+ external data
   sources (NASA EONET, NeoWs, USGS earthquakes, OpenAQ, GDACS).
   Most are public-free but some throttle aggressively. Decide
   whether v1 ships with operator-supplied keys or with documented
   per-user key registration.
4. Run `release-matrix.yml` against the signed build.
5. Cut release tag per the promote workflow.

Estimated operator time once credentials are in hand: ~3 hours
including external API key sweep and notarization round-trip.

---

## Portfolio operating system instructions

| Aspect                | Posture                                                                                                                                                                                                                                                        |
| --------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Portfolio status      | `Release Frozen`                                                                                                                                                                                                                                               |
| Operator's own status | **"Internal beta ready"** per `docs/comms/closeout/final-closeout-report.md` on canonical main                                                                                                                                                                 |
| Review cadence        | Suspend overdue counting                                                                                                                                                                                                                                       |
| Resurface conditions  | (a) Apple signing credentials wired, (b) operator decides internal-beta vs public-release scope, (c) operator audits external API keys, or (d) operator opens a v1.1 scope packet                                                                              |
| Co-batch with         | Signing cluster: DesktopPEt / ContentEngine / AIGCCore / Relay / FreeLanceInvoice / Nexus / DeepTank / OPscinema / ShipKit / SignalFlow / PixelForge / DatabaseSchema / LegalDocsReview / WorkdayDebrief / TicketDashboard / **EarthPulse** — **now 16 repos** |
| Special concern       | **Pre-shipped closeout docs.** Operator already wrote the readiness report — disposition doc should defer to it, not duplicate it.                                                                                                                             |
| Special concern       | **Internal-beta vs public-release** is a real second axis beyond signing.                                                                                                                                                                                      |

---

## Why this row is unusual in the cluster

Every other cluster member has the disposition document as the
first piece of release governance to land. EarthPulse already had
a richer closeout package shipped on canonical main when this
disposition pass ran. The disposition's job here is therefore:

1. **Acknowledge the operator's closeout report** as the primary
   release-readiness artifact (don't replace or override it).
2. Flag the **two extra release axes** (internal-beta-vs-public,
   external API key posture) that aren't in the standard signing
   playbook.
3. Slot the repo into the cluster so the operator sees it grouped
   with other Apple-signing dependencies during the credential
   round.

If a future round adds a repo with similar pre-shipped governance,
treat that as a positive signal — the operator is graduating from
"disposition tells me what state this is in" toward "I've already
told you the state, the disposition is for the portfolio OS."

---

## Reactivation procedure (for the next code session)

1. Verify `git branch -vv` shows `main` tracking `origin/main`.
   Already correct as of this disposition pass.
2. Re-read `docs/comms/closeout/final-closeout-report.md` first —
   the operator's own ledger is authoritative.
3. Review the local stash (`r9-earthpulse-stash`) — contains mods
   to `.codex/verify.commands`, perf workflows, perf scripts,
   AGENTS.md. Decide what belongs on `origin/main`.
4. Re-run `pnpm install && pnpm tauri build` to confirm toolchain.
5. **Audit external API key posture** before public release.
6. Decide internal-beta vs public-release scope before signing.

---

## Last known reference

| Field                  | Value                                                                                                                                                       |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `origin/main` tip      | `7ad1c8d` (HEAD)                                                                                                                                            |
| Closeout report        | `docs/comms/closeout/final-closeout-report.md` on `origin/main`                                                                                             |
| Architecture docs      | `docs/architecture/{data-flow,dependencies-and-external-services,frontend-backend-boundary,system-overview}.md` on `origin/main`                            |
| Default branch         | `main`                                                                                                                                                      |
| Build system           | Tauri 2 + Rust + JavaScript/TypeScript + Vite                                                                                                               |
| Release workflows      | `release-matrix.yml`, `release-promote.yml`, `quality-foundation.yml`, `quality-gates.yml`, `security-quality.yml`, `artifact-hygiene.yml` on `origin/main` |
| Release scaffolding    | **Already shipped** — including a full closeout report                                                                                                      |
| Blocker                | Apple signing + internal-beta-vs-public decision + external API key audit (operator-only)                                                                   |
| Migration state        | `legacy-origin` present but local tracking is correct; **zero orphans on `legacy-origin/main`**                                                             |
| External integrations  | NASA EONET, NeoWs (asteroids), USGS (earthquakes), OpenAQ (air quality), GDACS (disasters), ISS tracker — ~5+ public data feeds                             |
| Distinguishing feature | **Pre-shipped closeout report on canonical main** — first session repo with operator-declared closure already in tree                                       |
