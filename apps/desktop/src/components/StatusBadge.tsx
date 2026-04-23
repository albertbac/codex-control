import type { SessionStatus } from '../features/sessions/types';

const labels: Record<SessionStatus, string> = {
  working: 'Working',
  idle: 'Idle',
  waiting_approval: 'Waiting approval',
  errored: 'Errored',
  finished: 'Finished',
  unknown: 'Unknown',
};

export function StatusBadge({ status }: { status: SessionStatus }) {
  return <span className={`status-badge status-${status}`}>{labels[status]}</span>;
}
