import type { KeyboardEvent } from "react";
import {
  Ban,
  Copy,
  FileSearch,
  FolderSearch,
  GitBranch,
  GitCompare,
  SquareTerminal,
} from "lucide-react";
import type { DashboardSession } from "../features/sessions/types";
import { ApprovalPill } from "../features/approvals/ApprovalPill";
import { GitSummary } from "../features/git/GitSummary";
import { formatRelativeTime } from "../lib/time";
import { StatusBadge } from "./StatusBadge";

export interface SessionCardActions {
  onOpenTerminal(session: DashboardSession): void;
  onOpenEditor(session: DashboardSession): void;
  onCopySession(session: DashboardSession): void;
  onInspectTranscript(session: DashboardSession): void;
  onInspectDiff(session: DashboardSession): void;
  onTerminate(session: DashboardSession): void;
}

export function SessionCard({
  session,
  selected,
  onSelect,
  actions,
}: {
  session: DashboardSession;
  selected: boolean;
  onSelect(): void;
  actions: SessionCardActions;
}) {
  function handleKeyDown(event: KeyboardEvent<HTMLElement>) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      onSelect();
    }
  }

  return (
    <article
      className={`session-card ${selected ? "selected" : ""}`}
      role="button"
      tabIndex={0}
      onClick={onSelect}
      onKeyDown={handleKeyDown}
    >
      <div className="session-card-header">
        <div>
          <div className="session-card-title-row">
            <h3>{session.repoName ?? "Detached session"}</h3>
            <StatusBadge status={session.status} />
          </div>
          <div className="session-card-meta-row">
            <span className="meta-chip">
              <GitBranch size={14} /> {session.branch ?? "No branch"}
            </span>
            <span className="meta-chip">{session.model ?? "Model unavailable"}</span>
            <ApprovalPill value={session.approvalState} />
          </div>
        </div>
        <span className="muted">{formatRelativeTime(session.updatedAt)}</span>
      </div>

      <div className="session-card-body">
        <div>
          <p className="label">cwd</p>
          <p className="mono truncate">{session.cwd}</p>
        </div>
        <div>
          <p className="label">Last prompt</p>
          <p className="truncate-2">{session.lastPrompt ?? "No prompt captured yet."}</p>
        </div>
        <div>
          <p className="label">Last command</p>
          <p className="mono truncate">{session.lastCommand ?? "No shell command captured yet."}</p>
        </div>
      </div>

      <div className="session-card-footer">
        <div className="action-row">
          <button
            type="button"
            className="icon-button"
            onClick={(event) => {
              event.stopPropagation();
              actions.onOpenTerminal(session);
            }}
          >
            <SquareTerminal size={16} />
          </button>
          <button
            type="button"
            className="icon-button"
            onClick={(event) => {
              event.stopPropagation();
              actions.onOpenEditor(session);
            }}
          >
            <FolderSearch size={16} />
          </button>
          <button
            type="button"
            className="icon-button"
            onClick={(event) => {
              event.stopPropagation();
              actions.onCopySession(session);
            }}
          >
            <Copy size={16} />
          </button>
          <button
            type="button"
            className="icon-button"
            onClick={(event) => {
              event.stopPropagation();
              actions.onInspectTranscript(session);
            }}
            disabled={!session.transcriptPath}
          >
            <FileSearch size={16} />
          </button>
          <button
            type="button"
            className="icon-button"
            onClick={(event) => {
              event.stopPropagation();
              actions.onInspectDiff(session);
            }}
          >
            <GitCompare size={16} />
          </button>
          <button
            type="button"
            className="icon-button danger"
            onClick={(event) => {
              event.stopPropagation();
              actions.onTerminate(session);
            }}
            disabled={!session.process?.pid}
          >
            <Ban size={16} />
          </button>
        </div>
        <GitSummary
          changedFilesCount={session.changedFilesCount}
          stagedCount={session.stagedCount}
          unstagedCount={session.unstagedCount}
        />
      </div>
    </article>
  );
}
