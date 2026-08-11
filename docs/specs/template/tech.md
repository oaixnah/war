# Technical Spec: FEATURE_NAME

**Status:** Draft
**Owner:** OWNER
**Created:** YYYY-MM-DD
**Accepted:** Not accepted
**Product spec:** [product.md](product.md)

## Context

Describe the current implementation and why it cannot satisfy the accepted product behavior. Reference
relevant files, types, tests, and accepted ADRs. Update references after the upstream source baseline
is selected.

## Proposed Changes

Describe the smallest implementation that satisfies the product invariants. Identify modules changed,
new or removed responsibilities, and dependency direction.

## Data Flow

Describe the end-to-end flow through state, events, provider boundaries, tool boundaries, and UI. Add a
small Mermaid diagram only when prose is insufficient.

## Interfaces and State

Document new or changed interfaces, persisted state, event variants, cancellation behavior, and error
ownership. Avoid complete code listings unless a signature is central to the decision.

## Security and Privacy

Cover:

- permission checks and trust boundaries;
- data sent to providers;
- secret handling and redaction;
- path and workspace boundaries;
- logs and persistence;
- abuse, cancellation, and resource limits.

## Alternatives Considered

Explain credible alternatives and why they are not selected. Do not invent weak alternatives solely to
justify the proposal.

## Risks and Mitigations

| Risk | Mitigation |
| --- | --- |
| FAILURE_OR_MAINTENANCE_RISK | CONCRETE_PREVENTION_OR_DETECTION |

## Testing and Validation

Map validation to every numbered product invariant.

| Product invariant | Verification |
| --- | --- |
| 1 | UNIT_CONTRACT_INTEGRATION_GUI_OR_MANUAL_TEST |

List required formatting, linting, automated test commands, and macOS manual verification.

## Rollout and Removal

Describe any migration, temporary flag, staged rollout, and deletion milestone. Write `Direct replacement`
when no compatibility path is required.

## Follow-ups

- EXPLICITLY_DEFERRED_WORK, or `None`
