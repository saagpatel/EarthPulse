import { create } from "zustand";

export interface PersistedSettings {
  user_lat?: number;
  user_lon?: number;
  mag_threshold?: number;
  proximity_km?: number;
  notify_earthquakes?: boolean;
  notify_aurora?: boolean;
  notify_volcanoes?: boolean;
  sonification_enabled?: boolean;
  ollama_model?: string;
}

export interface SettingsState {
  userLat: number;
  userLon: number;
  isOpen: boolean;
  notifyEarthquakes: boolean;
  notifyAurora: boolean;
  notifyVolcanoes: boolean;
  earthquakeMagThreshold: number;
  proximityRadius: number;
  sonificationEnabled: boolean;
  ollamaModel: string;
  setLocation: (lat: number, lon: number) => void;
  toggle: () => void;
  setNotifyEarthquakes: (v: boolean) => void;
  setNotifyAurora: (v: boolean) => void;
  setNotifyVolcanoes: (v: boolean) => void;
  setEarthquakeMagThreshold: (v: number) => void;
  setProximityRadius: (v: number) => void;
  setSonificationEnabled: (v: boolean) => void;
  toggleSonification: () => void;
  setOllamaModel: (v: string) => void;
  hydrate: (settings: PersistedSettings) => void;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  userLat: 37.3382,
  userLon: -121.8863,
  isOpen: false,
  notifyEarthquakes: true,
  notifyAurora: true,
  notifyVolcanoes: true,
  earthquakeMagThreshold: 5.0,
  proximityRadius: 500,
  sonificationEnabled: false,
  ollamaModel: "llama3.2",
  setLocation: (lat, lon) => set({ userLat: lat, userLon: lon }),
  toggle: () => set((s) => ({ isOpen: !s.isOpen })),
  setNotifyEarthquakes: (v) => set({ notifyEarthquakes: v }),
  setNotifyAurora: (v) => set({ notifyAurora: v }),
  setNotifyVolcanoes: (v) => set({ notifyVolcanoes: v }),
  setEarthquakeMagThreshold: (v) => set({ earthquakeMagThreshold: v }),
  setProximityRadius: (v) => set({ proximityRadius: v }),
  setSonificationEnabled: (v) => set({ sonificationEnabled: v }),
  toggleSonification: () => set((s) => ({ sonificationEnabled: !s.sonificationEnabled })),
  setOllamaModel: (v) => set({ ollamaModel: v }),
  hydrate: (settings) =>
    set((state) => ({
      userLat: settings.user_lat ?? state.userLat,
      userLon: settings.user_lon ?? state.userLon,
      earthquakeMagThreshold: settings.mag_threshold ?? state.earthquakeMagThreshold,
      proximityRadius: settings.proximity_km ?? state.proximityRadius,
      notifyEarthquakes: settings.notify_earthquakes ?? state.notifyEarthquakes,
      notifyAurora: settings.notify_aurora ?? state.notifyAurora,
      notifyVolcanoes: settings.notify_volcanoes ?? state.notifyVolcanoes,
      sonificationEnabled: settings.sonification_enabled ?? state.sonificationEnabled,
      ollamaModel: settings.ollama_model?.trim() || state.ollamaModel,
    })),
}));
