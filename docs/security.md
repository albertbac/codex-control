# Security

Codex Control is local-first visibility tooling for Codex CLI sessions.

It is not a security sandbox.

## Local storage

Session state is stored on the local machine.

Codex Control does not send session data to a hosted service. There is no telemetry enabled by default.

## Redaction

Sensitive values are redacted before persistence where they match known patterns.

Redaction covers common cases such as API key-like values, bearer credentials, authorization headers, private key blocks, environment-style secret values, long high-entropy tokens, cookies, and session-like values.

Redaction is a safety layer, not a guarantee. Review local data before sharing logs or reports.

## Hook limits

Hooks are useful guardrails. They are not complete enforcement.

`PreToolUse` and `PostToolUse` currently focus on Bash-shaped tool events. They do not cover every possible command path, generated script, non-shell tool, or external workflow.

`PostToolUse` runs after the action it observes. It cannot undo side effects.

## Approval behavior

Codex Control must not auto-approve permission requests by default.

The policy command denies known destructive requests and otherwise leaves Codex approval flow intact.

## Logs and errors

Public output should not contain raw hook payloads, full private paths, credentials, cookies, authorization headers, or unredacted command text.

Diagnostics should be short and sanitized.

## Data deletion

To delete local data, close the app and remove the local store or spool files configured for Codex Control.

If unsure, run:

~~~bash
codex-control-hook doctor
~~~

The doctor command should report status without printing sensitive values.

## Auditing hook configuration

Review the hook files you installed and confirm that they call the expected binary:

~~~bash
codex-control-hook ingest
codex-control-hook policy
~~~

Do not install hook files from an untrusted source.
