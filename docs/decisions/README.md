# Architecture Decision Records

Architecture Decision Records preserve the context and consequences of durable choices. They explain
why the repository has a constraint; `docs/ARCHITECTURE.md` describes the resulting target state.

## When an ADR Is Required

Write an ADR when a decision:

- changes module or dependency direction;
- changes the network, context, permission, or secret boundary;
- selects or replaces a major framework, persistence model, protocol, or upstream strategy;
- creates a compatibility commitment or migration path;
- constrains multiple future features;
- would be expensive or risky to reverse.

Do not write an ADR for routine implementation details or choices local to one small function.

## Naming

Use a four-digit sequence and a short slug:

```text
0001-derive-from-warp-client.md
0002-local-first-agent-runtime.md
```

Copy [`template.md`](template.md) for a new record.

## Status

- `Proposed`: under discussion and not authoritative.
- `Accepted`: authoritative and reflected in architecture documentation.
- `Superseded`: replaced by a linked ADR.
- `Rejected`: considered but not selected.

Accepted ADRs are immutable historical records. If the decision changes, create a new ADR, mark the old
record superseded, and update `docs/ARCHITECTURE.md`.

## Index

| ADR | Status | Decision |
| --- | --- | --- |
| [0001](0001-derive-from-warp-client.md) | Accepted | Derive War from a fixed Warp client baseline |
| [0002](0002-local-first-agent-runtime.md) | Accepted | Build one local BYOK agent runtime with OpenAI Responses first |
