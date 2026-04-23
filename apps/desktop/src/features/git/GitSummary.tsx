export function GitSummary({
  changedFilesCount,
  stagedCount,
  unstagedCount,
}: {
  changedFilesCount: number;
  stagedCount: number;
  unstagedCount: number;
}) {
  return (
    <div className="session-card-stats">
      <span>{changedFilesCount} changed</span>
      <span>staged {stagedCount}</span>
      <span>unstaged {unstagedCount}</span>
    </div>
  );
}
