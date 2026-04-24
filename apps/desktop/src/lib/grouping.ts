import type { DashboardSession } from "../features/sessions/types";

export interface RepoGroup {
  name: string;
  sessions: DashboardSession[];
}

export function groupSessionsByRepo(sessions: DashboardSession[]): RepoGroup[] {
  const groups = new Map<string, DashboardSession[]>();
  for (const session of sessions) {
    const key = session.repoName ?? "No repository";
    const current = groups.get(key) ?? [];
    current.push(session);
    groups.set(key, current);
  }

  return Array.from(groups.entries())
    .map(([name, repoSessions]) => ({
      name,
      sessions: repoSessions.sort((left, right) => right.updatedAt.localeCompare(left.updatedAt)),
    }))
    .sort((left, right) => left.name.localeCompare(right.name));
}
