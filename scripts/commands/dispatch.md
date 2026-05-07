# /dispatch — audit-first code dispatch

When invoked via `/dispatch <task description>`, do the following:

1. **Run audit pre-flight.** Execute `hammerstein --template audit-this-plan --log /tmp/hc.jsonl "<task description>"` and display the Plain English summary to the user.

2. **Surface risks.** If the audit identifies scope creep, missed dependencies, or architectural concerns, state them clearly before proceeding.

3. **Confirm.** Ask the user: "Proceed with dispatch? (y/n)"

4. **If yes, dispatch via hd.** Execute `hd "<task description>"` and stream its output. `hd` handles provider routing and dispatches to aider (or the configured execution provider) for file editing and git operations.

5. **Report.** When `hd` completes, report the outcome: files changed, commits created, any issues encountered.

## Rules

- The audit is consultation, not a gate. If the user says proceed despite audit concerns, proceed.
- For trivial tasks (single-file rename, typo fix), ask if the user wants to skip the audit: "Skip audit for this? (y/n)" 
- Never modify the task description the user provides. Pass it verbatim to both `hammerstein` and `hd`.
- `hd` owns file editing, conversation state, and git operations. Don't duplicate those.
