# EarthPulse

[![TypeScript](https://img.shields.io/badge/TypeScript-%233178c6?style=flat-square&logo=typescript)](#) [![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](#)

> Leave it open on a second screen and the world starts moving — earthquakes, satellites, aurora, wildfires, and asteroid flybys, all in one place.

EarthPulse turns your desktop into a live globe of planetary activity. It aggregates 12 data streams — USGS earthquakes, ISS and satellite tracks, NOAA space weather, NASA wildfire/storm alerts, asteroid close approaches, and more — and presents them as a layered, explorable map you can scrub through time, filter by event type, or export to CSV and GeoJSON.

## Features

- **12 Live Data Layers** — Earthquakes (circles + heatmap), ISS tracker, satellite orbital tracks, aurora Kp index, volcanoes, GDACS hazard alerts, NASA wildfires and storms, asteroid close approaches, solar flares and CMEs, tectonic boundaries, meteor showers
- **24h Replay** — Scrub through the last 24 hours of seismic activity and watch events pulse across the globe in sequence
- **Historical Explorer** — Query USGS historical windows to compare past activity patterns against the present
- **Custom Watchlists** — Save locations you care about and receive proximity-based alerts when events occur nearby
- **Stats Dashboard** — Magnitude distributions, frequency trends, and Kp history in compact inline charts
- **Export** — CSV, GeoJSON, and screenshot export for any current map view

## Data Sources

| Layer | Source | Refresh |
|-------|--------|---------|
| Earthquakes | USGS GeoJSON | 60s |
| ISS tracker | Open Notify | 5s |
| Satellite tracks | CelesTrak TLE + SGP4 | 5min |
| Aurora / Kp index | NOAA SWPC | 15min |
| Volcanoes | Smithsonian GVP | 6h |
| GDACS hazard alerts | GDACS RSS | 15min |
| Wildfires + storms | NASA EONET v3 | 30min |
| Asteroid close approaches | NASA NEO API | 6h |
| Solar flares + CMEs | NASA DONKI | 3h |

## Quick Start

### Prerequisites

- Node.js 20+
- pnpm 9+
- Rust toolchain (stable) + Tauri v2 prerequisites for macOS

### Installation

```bash
git clone https://github.com/saagpatel/EarthPulse.git
cd EarthPulse
pnpm install
cp .env.example .env
```

### Run (development)

```bash
pnpm dev
```

### Build (desktop app)

```bash
pnpm build
```

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop shell | Tauri 2 + Rust |
| Frontend | React 19 + TypeScript + Vite |
| Map rendering | Leaflet / MapLibre |
| Satellite math | SGP4 propagator (Rust) |
| Storage | SQLite (watchlists, history) |
| Styling | Tailwind CSS |

## Architecture

EarthPulse is a Tauri 2 desktop app. The Rust backend manages all data fetching (polling each source on its configured interval), SGP4 satellite orbit propagation, SQLite persistence for watchlists and replay data, and the historical query engine. The React frontend renders the layered map, handles timeline scrubbing for the 24h replay, and drives the stats dashboard with data streamed from the Rust layer via Tauri commands.

## License

MIT
