# Target Architecture

**Status:** Accepted target architecture
**Last reviewed:** 2026-08-12

## Current State

The fixed Warp source baseline has been imported and inventoried in [`BASELINE.md`](BASELINE.md). The
inherited client builds and launches, but still contains hosted authentication, Warp Server, GraphQL,
Drive, telemetry, cloud agents, orchestration, editor, and LSP dependencies. This document defines the
target state; the imported source does not yet satisfy these boundaries.

## Context

Warp's open-source client provides a mature Rust terminal and custom UI framework, but its product
layer also includes hosted authentication, GraphQL clients, Drive, billing, cloud agents, orchestration,
an embedded editor, and many runtime feature flags. Warp's built-in agent harness and server are not
part of the open-source client.

War retains the terminal foundation while replacing the AI runtime and removing server-backed product
surfaces.

## Dependency Direction

```text
UI ───────> Application ───────> Terminal
                  │
                  └────────────> Agent Runtime
                                      │
                            ┌─────────┴─────────┐
                            v                   v
                     Provider Adapter      Tool Gateway
                            │                   │
                       OpenAI API         Filesystem / PTY
```

Dependencies point downward. In particular:

- Terminal has no knowledge of agents, models, prompts, or providers.
- Agent Runtime consumes terminal context through narrow application-owned interfaces.
- UI renders state and emits intent; it does not implement provider protocols or tool execution.
- Provider Adapter converts provider requests and streamed responses into internal agent events.
- Tool Gateway owns authorization and all agent-requested side effects.

## Components

### Terminal

The terminal subsystem owns PTY lifecycle, shell integration, terminal emulation, input, command block
construction, alternate-screen behavior, and terminal rendering state. Existing Warp code should be
retained where it is independent of excluded product services.

### Application

The application layer owns tabs, active working directory, timeline ordering, context attachments,
session lifecycle, and coordination between a terminal session and one agent session. It is the only
layer allowed to combine terminal and agent state.

### Agent Runtime

The runtime owns conversation state, model turns, tool-call sequencing, cancellation, retries that are
safe to repeat, context budgeting, and conversion between provider-neutral events and application
events.

The runtime must not use Warp server output IDs, GraphQL models, usage credits, workspace entitlements,
cloud handoff, or orchestration types.

### Provider Adapter

The first adapter supports OpenAI Responses API. The provider boundary should be small enough to add a
future adapter, but the MVP must not create a generalized routing platform.

The adapter owns:

- authentication headers and endpoint configuration;
- request serialization;
- streaming transport and event parsing;
- provider error classification;
- provider-specific continuation identifiers;
- conversion to provider-neutral agent events.

It never reads files, executes commands, mutates conversation UI, or decides permissions.

### Tool Gateway

The gateway exposes the MVP tool set: read files, glob paths, grep text, execute a shell command, and
apply a patch. Every tool declares whether it is read-only or mutating, but the gateway remains the
authority rather than trusting a model-supplied declaration.

The gateway owns validation, workspace path boundaries, approval state, execution, cancellation,
output limits, redaction, and result serialization.

## Agent Event Model

The internal stream should be intentionally small:

```text
TurnStarted
TextDelta
ReasoningStatus
ToolRequested
ToolAwaitingApproval
ToolStarted
ToolCompleted
ToolRejected
TurnCompleted
TurnFailed
TurnCancelled
```

Provider events are translated at the adapter boundary. UI models consume internal events and never
parse provider wire payloads.

## Context Model

Context is composed from explicit references rather than a hidden dump of application state.

Automatically available metadata is limited to low-risk values required for basic operation, such as
the platform, shell type, working directory, repository root, Git branch, and concise Git status.

Potentially sensitive content requires an explicit attachment or a tool request. This includes command
output, terminal history, file contents, diffs, selections, and images. Environment variables are not
context and must not be sent wholesale.

Every provider request passes through redaction and size limits. Redaction reduces accidental exposure
but does not replace explicit context selection.

## Permission Model

For the MVP:

- file reads, glob, and grep may run automatically within the active repository boundary;
- shell commands always require explicit approval;
- patches and all file mutations always require explicit approval;
- access outside the active repository requires explicit approval;
- a rejected tool call produces a structured result so the agent can continue safely;
- cancellation propagates from UI to provider stream and active tool execution;
- no heuristic may silently upgrade a mutating operation to read-only.

Future permission profiles require a product spec and ADR because they alter the trust model.

## Secrets and Persistence

- API keys are stored in macOS Keychain and represented in memory by opaque handles where practical.
- Configuration files may contain provider names and model IDs, never raw API keys.
- Logs omit prompt bodies, tool outputs, headers, and credentials by default.
- Persisted conversations are local, versioned, and independently deletable.
- Redacted persisted content is not assumed safe to upload; War has no synchronization service.
- Pre-alpha persistence formats have no compatibility guarantee unless a spec explicitly grants one.

## Network Boundary

The default application has no required startup network request. Model requests are initiated only by
an explicit agent action and flow through the active provider adapter. Future update checks or external
integrations require their own documented boundary and cannot be added incidentally.

M2 must include a runtime network audit proving that no Warp endpoint, authentication endpoint,
telemetry endpoint, or cloud agent endpoint is contacted.

## Dependency Policy

- Reuse an imported crate only when its retained responsibility matches War's target boundary.
- Do not keep an excluded product subsystem merely because another retained module imports one type.
- Prefer moving a small neutral type to a lower layer over retaining a server-oriented crate.
- New dependency versions belong in the workspace manifest.
- New dependencies require license compatibility and maintenance review.
- Cargo features represent real optional capabilities or short migrations, not permanent product
  configuration matrices.
- A temporary feature must identify its removal milestone.

## Upstream Strategy

War is based on a recorded Warp commit and does not continuously merge Warp `master`. The upstream
remote remains available for inspection. Relevant terminal, rendering, shell, platform, and security
fixes are cherry-picked individually.

The pre-upstream repository history is preserved by the `pre-upstream-foundation` tag. War foundation
documents were replayed as a commit on top of the selected baseline before `main` migration.

## Testing Boundaries

- Terminal behavior is tested without a model provider.
- Agent Runtime is tested with deterministic provider and tool fakes.
- Provider parsing is tested with sanitized stream fixtures.
- Tool Gateway is tested for path escape, cancellation, denial, output limits, and redaction.
- Application integration tests prove timeline ordering and permission transitions.
- Live provider calls are excluded from CI.

## Architecture Change Rule

Changes to dependency direction, network boundaries, context defaults, permission defaults, secret
storage, or provider ownership require an accepted ADR and corresponding updates to this document.
