import type { SettingsInfo } from "../sessions/types";
import { formatTimestamp } from "../../lib/time";

export function SettingsPanel({
  settings,
  usingFallback,
}: {
  settings: SettingsInfo;
  usingFallback: boolean;
}) {
  return (
    <section className="panel settings-panel">
      <div className="panel-header">
        <div>
          <p className="eyebrow">Settings</p>
          <h2>Local runtime</h2>
        </div>
        <span className={`status-dot ${settings.hookCliAvailable ? "ok" : "warn"}`}>
          {settings.hookCliAvailable ? "Hook CLI found" : "Hook CLI missing"}
        </span>
      </div>

      <div className="settings-grid">
        <div>
          <p className="label">Data directory</p>
          <p className="mono truncate">{settings.paths.dataDir}</p>
        </div>
        <div>
          <p className="label">Database</p>
          <p className="mono truncate">{settings.paths.databasePath}</p>
        </div>
        <div>
          <p className="label">Spool</p>
          <p className="mono truncate">{settings.paths.spoolPath}</p>
        </div>
        <div>
          <p className="label">Store mode</p>
          <p>{settings.storeMode}</p>
        </div>
        <div>
          <p className="label">Last ingest</p>
          <p>{formatTimestamp(settings.lastIngestAt)}</p>
        </div>
        <div>
          <p className="label">Hook CLI path</p>
          <p className="mono truncate">{settings.hookCliPath ?? "Unavailable"}</p>
        </div>
      </div>

      <div className="settings-note">
        <p className="label">Install snippet</p>
        <pre>{settings.hookInstallSnippet}</pre>
      </div>

      <div className="settings-note">
        <p className="label">Notes</p>
        <ul>
          {settings.notes.map((note) => (
            <li key={note}>{note}</li>
          ))}
          {usingFallback ? (
            <li>
              Fallback sample data is active. No live hook events have been observed in this
              runtime.
            </li>
          ) : null}
        </ul>
      </div>
    </section>
  );
}
