export function ApprovalPill({
  value,
}: {
  value: string | null;
}) {
  if (!value) {
    return <span className="meta-chip">No pending approvals</span>;
  }

  return <span className={`meta-chip approval-${value}`}>{value.split('_').join(' ')}</span>;
}
