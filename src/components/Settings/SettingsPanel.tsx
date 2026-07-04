import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useSettingsStore } from "../../stores/settingsStore";

export function SettingsPanel() {
  const store = useSettingsStore();

  if (!store.isOpen) return null;

  return <SettingsPanelContent store={store} />;
}

function SettingsPanelContent({
  store,
}: {
  store: ReturnType<typeof useSettingsStore>;
}) {
  // Local state for all editable fields — snapshot store values when panel opens
  const [lat, setLat] = useState(() => store.userLat.toString());
  const [lon, setLon] = useState(() => store.userLon.toString());
  const [magThreshold, setMagThreshold] = useState(
    () => store.earthquakeMagThreshold,
  );
  const [proximityRadius, setProximityRadius] = useState(
    () => store.proximityRadius,
  );
  const [notifyEq, setNotifyEq] = useState(() => store.notifyEarthquakes);
  const [notifyAurora, setNotifyAurora] = useState(() => store.notifyAurora);
  const [notifyVolc, setNotifyVolc] = useState(() => store.notifyVolcanoes);
  const [sonificationEnabled, setSonificationEnabled] = useState(
    () => store.sonificationEnabled,
  );
  const [ollamaModel, setOllamaModel] = useState(() => store.ollamaModel);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [isSaving, setIsSaving] = useState(false);
  const dialogRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      if (event.key === "Escape" && !isSaving) {
        store.toggle();
      }
    };

    document.addEventListener("keydown", onKeyDown);
    return () => document.removeEventListener("keydown", onKeyDown);
  }, [isSaving, store]);

  const handleSave = async () => {
    setSaveError(null);
    const parsedLat = parseFloat(lat);
    const parsedLon = parseFloat(lon);
    const sanitizedModel = ollamaModel.trim();
    const validCoords =
      Number.isFinite(parsedLat) &&
      Number.isFinite(parsedLon) &&
      parsedLat >= -90 &&
      parsedLat <= 90 &&
      parsedLon >= -180 &&
      parsedLon <= 180;

    if (!sanitizedModel) {
      setSaveError("Ollama model is required");
      return;
    }

    if (isSaving) return;

    if (validCoords) {
      store.setLocation(parsedLat, parsedLon);
    }
    store.setEarthquakeMagThreshold(magThreshold);
    store.setProximityRadius(proximityRadius);
    store.setNotifyEarthquakes(notifyEq);
    store.setNotifyAurora(notifyAurora);
    store.setNotifyVolcanoes(notifyVolc);
    store.setSonificationEnabled(sonificationEnabled);
    store.setOllamaModel(sanitizedModel);

    // Persist all runtime settings to backend
    setIsSaving(true);
    try {
      await invoke("save_settings", {
        settings: {
          user_lat: validCoords ? parsedLat : store.userLat,
          user_lon: validCoords ? parsedLon : store.userLon,
          mag_threshold: magThreshold,
          proximity_km: proximityRadius,
          notify_earthquakes: notifyEq,
          notify_aurora: notifyAurora,
          notify_volcanoes: notifyVolc,
          sonification_enabled: sonificationEnabled,
          ollama_model: sanitizedModel,
        },
      });

      store.toggle();
    } catch (e) {
      console.error("Failed to save settings:", e);
      setSaveError(String(e));
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <div
      className="fixed inset-0 bg-black/50 flex items-center justify-center z-[10000]"
      onClick={() => {
        if (!isSaving) {
          store.toggle();
        }
      }}
    >
      <div
        ref={dialogRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby="settings-title"
        className="bg-gray-900 border border-gray-700 rounded-lg p-6 w-96 space-y-5 max-h-[80vh] overflow-y-auto"
        onClick={(event) => event.stopPropagation()}
      >
        <h2 id="settings-title" className="text-lg font-bold">
          Settings
        </h2>

        <div className="space-y-3">
          <h3 className="text-sm font-semibold text-gray-300">Location</h3>
          <div className="grid grid-cols-2 gap-3">
            <div>
              <label className="text-xs text-gray-400 block mb-1">
                Latitude
              </label>
              <input
                id="settings-latitude"
                type="number"
                aria-label="Latitude"
                value={lat}
                onChange={(e) => setLat(e.target.value)}
                step="0.0001"
                className="w-full bg-gray-800 border border-gray-700 rounded px-3 py-1.5 text-sm"
              />
            </div>
            <div>
              <label className="text-xs text-gray-400 block mb-1">
                Longitude
              </label>
              <input
                id="settings-longitude"
                type="number"
                aria-label="Longitude"
                value={lon}
                onChange={(e) => setLon(e.target.value)}
                step="0.0001"
                className="w-full bg-gray-800 border border-gray-700 rounded px-3 py-1.5 text-sm"
              />
            </div>
          </div>
        </div>

        <div className="border-t border-gray-800" />

        <div className="space-y-3">
          <h3 className="text-sm font-semibold text-gray-300">Notifications</h3>
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={notifyEq}
              onChange={(e) => setNotifyEq(e.target.checked)}
              className="accent-red-500"
            />
            <span className="text-sm">Earthquake alerts</span>
          </label>

          <div className="ml-6 space-y-2">
            <div>
              <label className="text-xs text-gray-400 block mb-1">
                Global threshold: M{magThreshold.toFixed(1)}+
              </label>
              <input
                type="range"
                min={3}
                max={8}
                step={0.5}
                aria-label="Global earthquake alert threshold"
                value={magThreshold}
                onChange={(e) => setMagThreshold(Number(e.target.value))}
                className="w-full accent-red-500"
              />
            </div>
            <div>
              <label className="text-xs text-gray-400 block mb-1">
                Proximity: {proximityRadius} km
              </label>
              <input
                type="range"
                min={100}
                max={2000}
                step={100}
                aria-label="Alert proximity radius"
                value={proximityRadius}
                onChange={(e) => setProximityRadius(Number(e.target.value))}
                className="w-full accent-red-500"
              />
            </div>
          </div>

          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={notifyAurora}
              onChange={(e) => setNotifyAurora(e.target.checked)}
              className="accent-green-500"
            />
            <span className="text-sm">Aurora / Kp alerts</span>
          </label>

          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={notifyVolc}
              onChange={(e) => setNotifyVolc(e.target.checked)}
              className="accent-orange-500"
            />
            <span className="text-sm">Volcano alerts</span>
          </label>
        </div>

        <div className="border-t border-gray-800" />

        <div className="space-y-3">
          <h3 className="text-sm font-semibold text-gray-300">Audio</h3>
          <label className="flex items-center gap-2 cursor-pointer">
            <input
              type="checkbox"
              checked={sonificationEnabled}
              onChange={(e) => setSonificationEnabled(e.target.checked)}
              className="accent-purple-500"
            />
            <span className="text-sm">Sonification mode</span>
          </label>
          <p className="text-xs text-gray-500 ml-6">
            Play tones for new earthquakes and ambient drone based on Kp index
          </p>
        </div>

        <div className="border-t border-gray-800" />

        <div className="space-y-3">
          <h3 className="text-sm font-semibold text-gray-300">AI Summary</h3>
          <div>
            <label className="text-xs text-gray-400 block mb-1">
              Ollama model
            </label>
            <input
              id="settings-ollama-model"
              type="text"
              aria-label="Ollama model"
              value={ollamaModel}
              onChange={(e) => setOllamaModel(e.target.value)}
              className="w-full bg-gray-800 border border-gray-700 rounded px-3 py-1.5 text-sm"
              placeholder="llama3.2"
            />
          </div>
        </div>

        <div className="flex gap-2 justify-end">
          {saveError && (
            <div className="text-xs text-red-400 mr-auto self-center">
              {saveError}
            </div>
          )}
          <button
            onClick={store.toggle}
            disabled={isSaving}
            className="px-3 py-1.5 text-sm rounded bg-gray-800 hover:bg-gray-700"
          >
            Cancel
          </button>
          <button
            onClick={handleSave}
            disabled={isSaving}
            className="px-3 py-1.5 text-sm rounded bg-blue-600 hover:bg-blue-500 disabled:opacity-60"
          >
            {isSaving ? "Saving..." : "Save"}
          </button>
        </div>
      </div>
    </div>
  );
}
