# Feature Specs

Specs make non-trivial behavior and implementation boundaries reviewable before code is written. War
uses specs selectively; routine maintenance should not become paperwork.

## When a Spec Is Required

Write both a product spec and a technical spec when a change:

- introduces or materially changes user-visible behavior;
- adds an agent tool or changes tool approval behavior;
- changes what data is sent to a provider or persisted locally;
- changes terminal and agent interaction;
- crosses an architecture boundary;
- introduces a migration or compatibility commitment;
- cannot be validated clearly in a focused pull request description.

A spec is usually unnecessary for:

- documentation corrections;
- dependency pins with no behavior change;
- localized refactors that preserve boundaries;
- reproducible bug fixes whose expected behavior is already defined;
- test-only improvements.

When uncertain, write a short spec. A useful two-page decision is cheaper than a broad implementation
that solves the wrong problem.

## Directory Layout

Use a four-digit sequence and a short descriptive slug:

```text
docs/specs/0001-local-terminal-startup/
├── product.md
└── tech.md
```

Copy the files under [`template/`](template/) and replace all instructional text. The sequence is
repository-wide and does not depend on a GitHub issue number.

## Status

Every spec declares one status:

- `Draft`: open for design changes; implementation must not begin.
- `Accepted`: behavior and approach are approved for implementation.
- `Implemented`: the accepted behavior is present and verified.
- `Superseded`: replaced by a linked spec or decision.

Only accepted specs authorize implementation. A spec may be accepted in the same pull request that
starts implementation, but its product behavior should be reviewed first.

## Product Spec Rules

The product spec defines observable behavior, not Rust types or file changes. It includes:

- the user problem;
- goals and non-goals;
- numbered, testable behavior invariants;
- loading, empty, failure, cancellation, offline, and permission states where relevant;
- privacy and context expectations;
- success criteria;
- unresolved product questions.

## Technical Spec Rules

The technical spec is grounded in the current repository. It includes:

- current behavior and relevant files;
- module and dependency changes;
- state, events, interfaces, and data flow;
- security, privacy, concurrency, and migration risks;
- test coverage mapped to product invariants;
- manual validation;
- explicit follow-ups that are not required for the current slice.

Avoid designing abstraction layers for hypothetical future providers, tools, or platforms. Safety and
permission boundaries are allowed before a second implementation because they enforce product trust.

## Lifecycle

1. Create product and technical specs with `Draft` status.
2. Resolve product behavior before implementation details.
3. Confirm alignment with `docs/PRODUCT.md`, accepted ADRs, and `docs/ARCHITECTURE.md`.
4. Mark both specs `Accepted` and record the acceptance date.
5. Implement the smallest complete vertical slice.
6. Update specs when implementation reveals an incorrect assumption; do not silently diverge.
7. Mark specs `Implemented` only after their success criteria and validation pass.

Specs describe the resulting behavior. Historical discussion belongs in commits and pull requests.
