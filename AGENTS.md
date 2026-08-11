# AGENTS.md

This file defines repository-wide instructions for coding agents working on War.

## Source of Truth

Read these documents before making a non-trivial change:

1. `docs/PRODUCT.md` defines what the product is and is not.
2. Accepted records in `docs/decisions/` explain durable technical choices.
3. `docs/ARCHITECTURE.md` defines target module boundaries and safety constraints.
4. An accepted spec under `docs/specs/` defines the behavior of a scoped feature.
5. `docs/ROADMAP.md` defines delivery order, not product truth.
6. `DEVELOPMENT.md` defines the development workflow and quality gates.

If these documents conflict, stop and resolve the conflict in documentation rather than guessing.

## Current State

War is entering M1. The Warp source tree has not been imported yet, so application build and test
commands are not currently available. M1 must import and validate an exact upstream baseline before
War product code is changed.

## Product Invariants

- A local terminal must work without login or network access.
- War must not depend on Warp Server, Oz, Drive, hosted authentication, or billing.
- Shell and agent events belong to the same timeline.
- Agent mode is entered explicitly.
- The first supported platform is macOS.
- The first supported provider is OpenAI Responses API with a user-owned API key.
- The first release supports one local agent, not orchestration or cloud execution.
- Secrets must not appear in logs, persisted prompts, or telemetry. War has no product telemetry.
- Shell commands and file mutations requested by the agent require explicit approval in the MVP.

## Architecture Rules

- Terminal code must not depend on agent or provider code.
- Provider adapters perform model protocol translation only; they never execute tools.
- All agent-requested side effects pass through one tool permission gateway.
- New agent code must not reuse Warp server IDs, GraphQL models, quota models, or cloud handoff types.
- Outbound model traffic is isolated behind the provider boundary.
- Do not send the complete environment or terminal history to a model.
- Prefer deletion over maintaining parallel old and new implementations.
- Feature flags require an owner, purpose, and removal milestone.

Temporary violations inherited from Warp may exist after M1. Do not build new behavior on those
violations; remove them in the relevant roadmap milestone.

## Change Workflow

- Keep one primary work item active at a time.
- Use the smallest change that completes a testable vertical slice.
- Follow the spec threshold in `docs/specs/README.md`.
- Use an ADR for durable decisions that constrain future implementations.
- Do not add backward compatibility before a shipped behavior or persisted format requires it.
- Do not add abstractions for hypothetical providers, tools, or platforms. Safety boundaries are the
  exception.
- Update the roadmap only when milestone scope or evidence changes.

## Code Standards

These rules apply after the upstream source import:

- Follow the repository Rust toolchain and Rust 2024 edition.
- Run the repository formatter; do not hand-format around it.
- Treat Clippy warnings as errors.
- Put workspace dependency versions in the workspace manifest.
- Avoid `unwrap` and `expect` for user input, provider responses, filesystem data, and network data.
- Add error context at subsystem boundaries without including sensitive values.
- Comments explain non-obvious reasons, invariants, or workarounds, not syntax.
- Tests accompany non-trivial logic and bug fixes.
- Preserve exhaustive matches where practical.
- Keep terminal model lock scopes short and never acquire the same model lock recursively.

## Verification

After M1, use the exact commands documented in `DEVELOPMENT.md`. A code change is not complete until
formatting, linting, relevant automated tests, and applicable macOS manual verification pass.

Documentation-only changes must at least be checked for valid relative links, consistent terminology,
and clean Markdown formatting.

## Git Hygiene

- Keep `main` releasable.
- Use short-lived branches named `feat/*`, `fix/*`, `refactor/*`, or `docs/*`.
- Do not mix unrelated cleanup into a feature change.
- Preserve upstream attribution and commit identity when cherry-picking upstream fixes.
- Never merge the full upstream branch merely to obtain one fix.
- Do not commit generated artifacts, credentials, local IDE state, or model transcripts.
