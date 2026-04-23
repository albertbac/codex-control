import { describe, expect, it } from 'vitest';
import { groupSessionsByRepo } from './grouping';

describe('groupSessionsByRepo', () => {
  it('groups sessions by repo name and sorts each group by updatedAt desc', () => {
    const groups = groupSessionsByRepo([
      {
        id: 'b',
        cwd: '/tmp/b',
        repoRoot: '/tmp/repo',
        repoName: 'repo',
        branch: 'main',
        model: null,
        transcriptPath: null,
        status: 'idle',
        lastPrompt: null,
        lastCommand: null,
        lastAssistantMessage: null,
        startedAt: '2026-04-23T10:00:00Z',
        updatedAt: '2026-04-23T10:00:00Z',
        approvalState: null,
        changedFilesCount: 0,
        stagedCount: 0,
        unstagedCount: 0,
        diffStat: null,
        transcriptPreview: null,
        process: null,
        source: 'test',
        isStale: false,
      },
      {
        id: 'a',
        cwd: '/tmp/a',
        repoRoot: '/tmp/repo',
        repoName: 'repo',
        branch: 'main',
        model: null,
        transcriptPath: null,
        status: 'working',
        lastPrompt: null,
        lastCommand: null,
        lastAssistantMessage: null,
        startedAt: '2026-04-23T11:00:00Z',
        updatedAt: '2026-04-23T11:00:00Z',
        approvalState: null,
        changedFilesCount: 0,
        stagedCount: 0,
        unstagedCount: 0,
        diffStat: null,
        transcriptPreview: null,
        process: null,
        source: 'test',
        isStale: false,
      },
    ]);

    expect(groups).toHaveLength(1);
    expect(groups[0].sessions.map((session) => session.id)).toEqual(['a', 'b']);
  });
});
