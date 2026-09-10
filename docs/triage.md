# Issue Triage

How incoming issues get labeled and prioritized.

## Labels

| Label | Meaning |
| --- | --- |
| `bug` | Confirmed incorrect behavior (wrong cost numbers, crash, bad parsing). |
| `enhancement` | New feature or improvement request. |
| `good first issue` | Small, self-contained, good entry point for new contributors. |
| `help wanted` | Maintainer doesn't plan to pick this up soon; contributions welcome. |
| `docs` | Documentation-only changes. |
| `question` | Usage question, not a bug/feature request. |
| `needs-repro` | Waiting on the reporter for a minimal reproduction. |
| `wontfix` | Out of scope or intentional behavior; closed with explanation. |

## Triage steps for a new issue

1. **Can it be reproduced from the description alone?**
   - If not, label `needs-repro` and ask for: `soroban-sdk` version, Rust toolchain version, and a minimal `record()` call or test crate.
2. **Is it a bug, feature request, or question?** Label accordingly.
3. **Bugs**: assess severity.
   - Misreported severity color (red/yellow/green) or incorrect percentages → high priority, affects the tool's core value proposition (see `SECURITY.md` — this overlaps with the "false negative" concern).
   - Cosmetic table/HTML formatting issues → normal priority.
4. **Feature requests**: check `docs/roadmap.md` first — if it's already planned, link the issue there instead of leaving it as a standalone request.
5. **Size it** if you're planning to label it for external contribution (e.g. for a Drips Wave or similar sprint): trivial / small / medium, based roughly on:
   - **Trivial** — single function, no new tests needed beyond the existing example, e.g. a new CLI flag.
   - **Small** — touches one crate, needs a small test addition.
   - **Medium** — touches the JSON protocol or spans both `harness/` and `cli/`.

## Response time targets (informal)

- Acknowledge new issues within a few days.
- `needs-repro` issues with no response from the reporter after ~2 weeks can be closed with a note that they're welcome to reopen with more detail.

## Closing issues

Always close with a comment explaining why (fixed in `vX.Y.Z`, out of scope, duplicate of #N, etc.) rather than closing silently.
