import { useEffect, useMemo, useState } from 'react';
import { RefreshCw, ShieldCheck } from 'lucide-react';
import { EmptyState } from '../components/EmptyState';
import { SessionCard, type SessionCardActions } from '../components/SessionCard';
import { groupSessionsByRepo } from '../lib/grouping';
import {
  copyToClipboard,
  inspectGitDiff,
  inspectTranscript,
  loadDashboard,
  loadSettings,
  loadTimeline,
  openEditor,
  openTerminal,
  terminateProcess,
} from '../lib/tauri';
import type { DashboardSession, SettingsInfo, TimelineItem } from '../features/sessions/types';
import { TimelinePanel } from '../features/timeline/TimelinePanel';
import { SettingsPanel } from '../features/settings/SettingsPanel';

export function App() {
  const [sessions, setSessions] = useState<DashboardSession[]>([]);
  const [selectedSessionId, setSelectedSessionId] = useState<string | null>(null);
  const [timeline, setTimeline] = useState<TimelineItem[]>([]);
  const [inspectResult, setInspectResult] = useState<string | null>(null);
  const [settings, setSettings] = useState<SettingsInfo | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [usingFallback, setUsingFallback] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const selectedSession = useMemo(
    () => sessions.find((session) => session.id === selectedSessionId) ?? null,
    [sessions, selectedSessionId],
  );

  useEffect(() => {
    let cancelled = false;

    async function refresh() {
      try {
        const [dashboard, settingsResult] = await Promise.all([loadDashboard(), loadSettings()]);
        if (cancelled) return;
        setSessions(dashboard.sessions);
        setUsingFallback(dashboard.usingFallback || settingsResult.usingFallback);
        setSettings(settingsResult.settings);
        setError(null);
        if (!selectedSessionId && dashboard.sessions.length > 0) {
          setSelectedSessionId(dashboard.sessions[0].id);
        }
      } catch (cause) {
        if (cancelled) return;
        setError(cause instanceof Error ? cause.message : 'Unexpected dashboard error');
      } finally {
        if (!cancelled) {
          setIsLoading(false);
        }
      }
    }

    void refresh();
    const interval = window.setInterval(() => void refresh(), 2500);
    return () => {
      cancelled = true;
      window.clearInterval(interval);
    };
  }, [selectedSessionId]);

  useEffect(() => {
    let cancelled = false;
    if (!selectedSessionId) {
      setTimeline([]);
      return;
    }

    async function refreshTimeline() {
      try {
        const sessionId = selectedSessionId;
        if (!sessionId) return;
        const items = await loadTimeline(sessionId);
        if (!cancelled) {
          setTimeline(items);
        }
      } catch (cause) {
        if (!cancelled) {
          setError(cause instanceof Error ? cause.message : 'Unexpected timeline error');
        }
      }
    }

    void refreshTimeline();
    return () => {
      cancelled = true;
    };
  }, [selectedSessionId]);

  const groups = useMemo(() => groupSessionsByRepo(sessions), [sessions]);

  const actions: SessionCardActions = {
    onOpenTerminal: (session) => void openTerminal(session.cwd).catch(handleActionError),
    onOpenEditor: (session) => void openEditor(session.cwd).catch(handleActionError),
    onCopySession: (session) => void copyToClipboard(session.id).catch(handleActionError),
    onInspectTranscript: (session) => {
      if (!session.transcriptPath) return;
      void inspectTranscript(session.transcriptPath)
        .then(setInspectResult)
        .catch(handleActionError);
    },
    onInspectDiff: (session) => {
      void inspectGitDiff(session.cwd)
        .then(setInspectResult)
        .catch(handleActionError);
    },
    onTerminate: (session) => {
      if (!session.process?.pid) return;
      const confirmed = window.confirm(`Terminate local Codex process ${session.process.pid}?`);
      if (!confirmed) return;
      void terminateProcess(session.process.pid)
        .then(() => setInspectResult(`Sent SIGTERM to process ${session.process?.pid}.`))
        .catch(handleActionError);
    },
  };

  function handleActionError(cause: unknown) {
    setError(cause instanceof Error ? cause.message : 'Unexpected action failure');
  }

  return (
    <main className="layout-shell">
      <header className="topbar">
        <div>
          <p className="eyebrow">Codex Control</p>
          <h1>Track local Codex sessions in one place</h1>
          <p className="subtitle">
            See current session state, recent commands, approvals, transcript references, and Git drift without leaving the machine.
          </p>
        </div>
        <div className="topbar-actions">
          <span className={`status-dot ${usingFallback ? 'warn' : 'ok'}`}>
            {usingFallback ? 'Fallback state' : 'Live local state'}
          </span>
          <button type="button" className="secondary-button" onClick={() => window.location.reload()}>
            <RefreshCw size={16} /> Refresh
          </button>
        </div>
      </header>

      {error ? (
        <section className="banner banner-error">{error}</section>
      ) : null}
      {usingFallback ? (
        <section className="banner banner-warn">
          <ShieldCheck size={16} /> Fallback sample data is visible because the local desktop runtime is unavailable.
        </section>
      ) : null}

      <section className="content-grid">
        <section className="panel sessions-panel">
          <div className="panel-header">
            <div>
              <p className="eyebrow">Dashboard</p>
              <h2>Sessions by repository</h2>
            </div>
            <span className="muted">{sessions.length} tracked sessions</span>
          </div>

          {isLoading ? (
            <EmptyState title="Loading local state" body="Reading local session history, process state, and repository metadata." />
          ) : groups.length === 0 ? (
            <EmptyState
              title="No sessions found"
              body="No Codex sessions were discovered yet. Install the hooks, open a workspace, and start a Codex CLI session."
            />
          ) : (
            <div className="repo-group-list">
              {groups.map((group) => (
                <div key={group.name} className="repo-group">
                  <div className="repo-group-header">
                    <h3>{group.name}</h3>
                    <span>{group.sessions.length} sessions</span>
                  </div>
                  <div className="repo-group-cards">
                    {group.sessions.map((session) => (
                      <SessionCard
                        key={session.id}
                        session={session}
                        selected={selectedSessionId === session.id}
                        onSelect={() => setSelectedSessionId(session.id)}
                        actions={actions}
                      />
                    ))}
                  </div>
                </div>
              ))}
            </div>
          )}
        </section>

        <TimelinePanel session={selectedSession} timeline={timeline} inspectResult={inspectResult} />
        {settings ? <SettingsPanel settings={settings} usingFallback={usingFallback} /> : null}
      </section>
    </main>
  );
}
