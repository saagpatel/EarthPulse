import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { SettingsPanel } from "./SettingsPanel";
import { useSettingsStore } from "../../stores/settingsStore";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("SettingsPanel", () => {
  it("opens with current settings without dispatching setter updates", () => {
    const setLocation = vi.fn();
    const setEarthquakeMagThreshold = vi.fn();
    const setProximityRadius = vi.fn();
    const setNotifyEarthquakes = vi.fn();
    const setNotifyAurora = vi.fn();
    const setNotifyVolcanoes = vi.fn();
    const setSonificationEnabled = vi.fn();
    const setOllamaModel = vi.fn();

    useSettingsStore.setState({
      isOpen: true,
      userLat: 12.3456,
      userLon: -98.7654,
      notifyEarthquakes: false,
      notifyAurora: true,
      notifyVolcanoes: false,
      earthquakeMagThreshold: 6.5,
      proximityRadius: 900,
      sonificationEnabled: true,
      ollamaModel: "llama3.3",
      setLocation,
      setEarthquakeMagThreshold,
      setProximityRadius,
      setNotifyEarthquakes,
      setNotifyAurora,
      setNotifyVolcanoes,
      setSonificationEnabled,
      setOllamaModel,
    });

    render(<SettingsPanel />);

    expect(screen.getByRole("dialog", { name: "Settings" })).toBeVisible();
    expect(screen.getByLabelText("Latitude")).toHaveValue(12.3456);
    expect(screen.getByLabelText("Longitude")).toHaveValue(-98.7654);
    expect(screen.getByLabelText("Ollama model")).toHaveValue("llama3.3");

    expect(setLocation).not.toHaveBeenCalled();
    expect(setEarthquakeMagThreshold).not.toHaveBeenCalled();
    expect(setProximityRadius).not.toHaveBeenCalled();
    expect(setNotifyEarthquakes).not.toHaveBeenCalled();
    expect(setNotifyAurora).not.toHaveBeenCalled();
    expect(setNotifyVolcanoes).not.toHaveBeenCalled();
    expect(setSonificationEnabled).not.toHaveBeenCalled();
    expect(setOllamaModel).not.toHaveBeenCalled();
  });
});
