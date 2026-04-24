import type { DashboardSession, TimelineItem } from "../sessions/types";
import { formatTimestamp } from "../../lib/time";

export function TimelinePanel({
  session,
  timeline,
  inspectResult,
}: {
  session: DashboardSession | null;
  timeline: TimelineItem[];
  inspectResult: string | null;
}) {
  return (
    <section className="panel timeline-panel">
      <div className="panel-header">
        <div>
          <p className="eyebrow">Timeline</p>
          <h2>{session ? session.id : "Select a session"}</h2>
        </div>
      </div>

      {!session ? (
        <div className="empty-state compact">
          <div>
            <h3>No session selected</h3>
            <p>
              Choose a session card to inspect prompts, commands, approvals, results, and transcript
              references.
            </p>
          </div>
        </div>
      ) : (
        <>
          <div className="timeline-meta">
            <span className="meta-chip">{session.repoName ?? "Detached workspace"}</span>
            <span className="meta-chip">{session.branch ?? "No branch"}</span>
            <span className="meta-chip">
              {session.process?.pid ? `pid ${session.process.pid}` : "no pid"}
            </span>
          </div>

          <div className="timeline-scroll">
            {timeline.length === 0 ? (
              <div className="empty-state compact">
                <div>
                  <h3>No timeline entries yet</h3>
                  <p>The session exists, but no persisted hook events were found yet.</p>
                </div>
              </div>
            ) : (
              timeline.map((item) => (
                <article key={item.id} className="timeline-item">
                  <div className="timeline-item-header">
                    <strong>{item.eventName}</strong>
                    <span>{formatTimestamp(item.createdAt)}</span>
                  </div>
                  {item.command ? <p className="mono">{item.command}</p> : null}
                  {item.approvalRequest ? <p>{item.approvalRequest}</p> : null}
                  {item.resultSummary ? <p>{item.resultSummary}</p> : null}
                  {item.transcriptPath ? (
                    <p className="mono truncate">{item.transcriptPath}</p>
                  ) : null}
                  {item.gitState ? <p>{item.gitState}</p> : null}
                </article>
              ))
            )}
          </div>

          <div className="inspect-pane">
            <p className="label">Inspect output</p>
            <pre>
              {inspectResult ??
                "Run an inspect action from a session card to preview transcript or Git diff output here."}
            </pre>
          </div>
        </>
      )}
    </section>
  );
}
