import { mockSessions, mockSettings, mockTimeline } from './mockData';
import type { DashboardSession, SettingsInfo, TimelineItem } from '../features/sessions/types';

async function tauriInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  const hasTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;
  if (!hasTauri) {
    throw new Error('Tauri runtime not available');
  }
  const { invoke } = await import('@tauri-apps/api/core');
  return invoke<T>(command, args);
}

export async function loadDashboard(): Promise<{ sessions: DashboardSession[]; usingMock: boolean }> {
  try {
    const sessions = await tauriInvoke<DashboardSession[]>('dashboard_snapshot');
    return { sessions, usingMock: false };
  } catch {
    return { sessions: mockSessions, usingMock: true };
  }
}

export async function loadTimeline(sessionId: string): Promise<TimelineItem[]> {
  try {
    return await tauriInvoke<TimelineItem[]>('session_timeline', { sessionId });
  } catch {
    return mockTimeline.filter((item) => item.sessionId === sessionId);
  }
}

export async function loadSettings(): Promise<{ settings: SettingsInfo; usingMock: boolean }> {
  try {
    const settings = await tauriInvoke<SettingsInfo>('settings_info');
    return { settings, usingMock: false };
  } catch {
    return { settings: mockSettings, usingMock: true };
  }
}

export async function openTerminal(cwd: string): Promise<void> {
  await tauriInvoke('open_terminal', { cwd });
}

export async function openEditor(cwd: string): Promise<void> {
  await tauriInvoke('open_editor', { cwd });
}

export async function inspectGitDiff(cwd: string): Promise<string> {
  return tauriInvoke<string>('inspect_git_diff', { cwd });
}

export async function inspectTranscript(path: string): Promise<string> {
  return tauriInvoke<string>('inspect_transcript', { path });
}

export async function copyToClipboard(value: string): Promise<void> {
  if (navigator.clipboard) {
    await navigator.clipboard.writeText(value);
    return;
  }
  throw new Error('Clipboard is not available in this runtime');
}

export async function terminateProcess(pid: number): Promise<void> {
  await tauriInvoke('terminate_process', { pid, confirm: true });
}
