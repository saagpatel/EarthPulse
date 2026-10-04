import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useSourceHealthStore } from "../../stores/sourceHealthStore";
import { SourceHealthPanel } from "./SourceHealthPanel";

describe("SourceHealthPanel", () => {
  const now = 1_800_000_000_000;

  beforeEach(() => {
    vi.spyOn(Date, "now").mockReturnValue(now);
    useSourceHealthStore.setState({ bySource: {} });
    delete window.__EARTHPULSE_BROWSER_MOCKS__;
  });

  afterEach(() => {
    cleanup();
    vi.restoreAllMocks();
    delete window.__EARTHPULSE_BROWSER_MOCKS__;
  });

  it("distinguishes empty native and browser preview telemetry", () => {
    const view = render(<SourceHealthPanel />);
    expect(
      screen.getByText("Collecting source telemetry..."),
    ).toBeInTheDocument();
    window.__EARTHPULSE_BROWSER_MOCKS__ = true;
    view.rerender(<SourceHealthPanel />);
    expect(
      screen.getByText("Waiting for preview telemetry..."),
    ).toBeInTheDocument();
  });

  it("shows recent, overdue, and failed sources with appropriate ages", () => {
    const upsert = useSourceHealthStore.getState().upsertEvent;
    upsert({ source: "iss", ok: true, timestamp_ms: now - 1_000 });
    upsert({ source: "earthquakes", ok: true, timestamp_ms: now - 180_000 });
    upsert({ source: "solar", ok: true, timestamp_ms: now - 7_200_000 });
    upsert({ source: "meteors", ok: true, timestamp_ms: now - 172_800_001 });
    upsert({
      source: "unknown-feed",
      ok: false,
      timestamp_ms: now,
      error: "Feed offline",
    });
    render(<SourceHealthPanel />);
    expect(screen.getByText("1s ago")).toBeInTheDocument();
    expect(screen.getByText("3m ago")).toBeInTheDocument();
    expect(screen.getByText("2h ago")).toBeInTheDocument();
    expect(screen.getByText("2d ago")).toBeInTheDocument();
    expect(screen.getByText("never")).toBeInTheDocument();
    expect(screen.getByText("Feed offline")).toBeInTheDocument();
    expect(screen.getAllByText("No recent successful update")).toHaveLength(3);
  });

  it("renders degraded stale messaging with elapsed time", () => {
    useSourceHealthStore.setState({
      bySource: {
        volcanoes: {
          source: "volcanoes",
          ok: true,
          degraded: true,
          degradedSince: Date.now() - 90_000,
          lastSuccessAt: Date.now() - 90_000,
          lastFailureAt: null,
          consecutiveFailures: 0,
          lastError: "Live feed unavailable; using curated fallback data",
        },
      },
    });

    render(<SourceHealthPanel />);

    expect(screen.getByText("Data Health")).toBeInTheDocument();
    expect(screen.getByText("volcanoes")).toBeInTheDocument();
    expect(screen.getByText(/fallback data/i)).toBeInTheDocument();
  });
});
