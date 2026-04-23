export type SessionStatus =
  | 'working'
  | 'idle'
  | 'waiting_approval'
  | 'errored'
  | 'finished'
  | 'unknown';

export interface CodexSession {
  id: string;
  cwd: string;
  repoRoot: string | null;
  repoName: string | null;
  branch: string | null;
  model: string | null;
  transcriptPath: string | null;
  status: SessionStatus;
  lastPrompt: string | null;
  lastCommand: string | null;
  lastAssistantMessage: string | null;
  startedAt: string;
  updatedAt: string;
}

export interface DashboardSession {
  id: string;
  cwd: string;
  repoRoot: string | null;
  repoName: string | null;
  branch: string | null;
  model: string | null;
  transcriptPath: string | null;
  status: SessionStatus;
  lastPrompt: string | null;
  lastCommand: string | null;
  lastAssistantMessage: string | null;
  startedAt: string;
  updatedAt: string;
  approvalState: string | null;
  changedFilesCount: number;
  stagedCount: number;
  unstagedCount: number;
  diffStat: string | null;
  transcriptPreview: string | null;
  process: {
    pid: number;
    parentPid: number | null;
    cwd: string;
    command: string;
    uptimeSeconds: number;
  } | null;
  source: string;
  isStale: boolean;
}

export interface TimelineItem {
  id: string;
  sessionId: string;
  eventName:
    | 'SessionStart'
    | 'UserPromptSubmit'
    | 'PreToolUse'
    | 'PermissionRequest'
    | 'PostToolUse'
    | 'Stop'
    | 'Unknown';
  createdAt: string;
  command: string | null;
  approvalRequest: string | null;
  resultSummary: string | null;
  transcriptPath: string | null;
  gitState: string | null;
  payload: unknown;
}

export interface SettingsInfo {
  paths: {
    dataDir: string;
    databasePath: string;
    spoolPath: string;
  };
  hookCliAvailable: boolean;
  hookCliPath: string | null;
  storeMode: string;
  lastIngestAt: string | null;
  hookInstallSnippet: string;
  notes: string[];
}
