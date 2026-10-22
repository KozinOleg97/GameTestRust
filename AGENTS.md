## Agent skills

### Issue tracker

Issues and specs live as GitHub issues on `KozinOleg97/GameTestRust`, managed with the `gh` CLI. See `docs/agents/issue-tracker.md`.

### Domain docs

Single-context layout: one `GLOSSARY.md` at the repo root plus decisions in `docs/adr/`. See `docs/agents/domain.md`.

### Workflow phases

- **Design & spec** (`.agents/skills`: `grilling`, `grill-with-docs`, `to-spec`, `to-tickets`, `domain-modeling`) — the mattpocock/skills set. Use it for planning, interviewing, writing specs and tickets.
- **Implementation** — use the `ponytail` skill (lazy senior dev mode: smallest complete change, no over-engineering) together with `implement`/`tdd` when actually writing code from a spec or ticket.
