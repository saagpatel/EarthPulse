import { beforeEach, describe, expect, it } from "vitest";
import { useSettingsStore } from "./settingsStore";

describe("settingsStore", () => {
  beforeEach(() => {
    useSettingsStore.setState(useSettingsStore.getInitialState(), true);
  });

  it("preserves zero, false, and omitted settings during hydration", () => {
    const state = useSettingsStore.getState();
    state.setLocation(12, 34);
    state.setEarthquakeMagThreshold(6);
    state.setProximityRadius(900);
    state.setSonificationEnabled(true);
    state.setOllamaModel("custom-model");
    state.hydrate({ user_lat: 0, notify_earthquakes: false, ollama_model: "  " });

    expect(useSettingsStore.getState()).toMatchObject({
      userLat: 0,
      userLon: 34,
      notifyEarthquakes: false,
      earthquakeMagThreshold: 6,
      proximityRadius: 900,
      sonificationEnabled: true,
      ollamaModel: "custom-model",
    });
  });

  it("applies user settings and toggles without overwriting other preferences", () => {
    const state = useSettingsStore.getState();
    state.setLocation(-20, 150);
    state.setNotifyEarthquakes(false);
    state.setNotifyAurora(false);
    state.setNotifyVolcanoes(false);
    state.setEarthquakeMagThreshold(4);
    state.setProximityRadius(250);
    state.setOllamaModel("local-model");
    state.toggle();
    state.toggleSonification();
    expect(useSettingsStore.getState()).toMatchObject({
      userLat: -20,
      userLon: 150,
      isOpen: true,
      notifyEarthquakes: false,
      notifyAurora: false,
      notifyVolcanoes: false,
      earthquakeMagThreshold: 4,
      proximityRadius: 250,
      sonificationEnabled: true,
      ollamaModel: "local-model",
    });
    state.toggle();
    state.toggleSonification();
    expect(useSettingsStore.getState()).toMatchObject({
      isOpen: false,
      sonificationEnabled: false,
      ollamaModel: "local-model",
    });
  });

  it("hydrates persisted settings into runtime state", () => {
    useSettingsStore.setState({
      userLat: 0,
      userLon: 0,
      notifyEarthquakes: false,
      notifyAurora: false,
      notifyVolcanoes: false,
      earthquakeMagThreshold: 3,
      proximityRadius: 100,
      sonificationEnabled: false,
      ollamaModel: "baseline",
    });

    useSettingsStore.getState().hydrate({
      user_lat: 40.71,
      user_lon: -74.0,
      notify_earthquakes: true,
      notify_aurora: true,
      notify_volcanoes: true,
      mag_threshold: 5.5,
      proximity_km: 800,
      sonification_enabled: true,
      ollama_model: "llama3.3",
    });

    const state = useSettingsStore.getState();
    expect(state.userLat).toBe(40.71);
    expect(state.userLon).toBe(-74.0);
    expect(state.notifyEarthquakes).toBe(true);
    expect(state.notifyAurora).toBe(true);
    expect(state.notifyVolcanoes).toBe(true);
    expect(state.earthquakeMagThreshold).toBe(5.5);
    expect(state.proximityRadius).toBe(800);
    expect(state.sonificationEnabled).toBe(true);
    expect(state.ollamaModel).toBe("llama3.3");
  });
});
