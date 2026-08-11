# ADR 0002: Build a Local BYOK Agent Runtime

**Status:** Accepted
**Date:** 2026-08-11
**Decision owners:** War maintainer

## Context

Warp's client contains Agent Mode presentation and protocol-related types, but the Warp agent harness,
Oz orchestration, and hosted services are not part of the open-source client. Reusing the existing AI
path would keep War dependent on services it does not control and would conflict with offline terminal,
no-login, and user-owned credential requirements.

An external agent protocol could reduce implementation effort, but it would delegate core context,
permission, and lifecycle behavior to separately installed tools. War's product direction calls for a
native, coherent agent experience whose trust boundaries are enforced by the terminal application.

Supporting several providers in the first release would multiply stream, tool-call, error, reasoning,
and test behavior before the runtime contract is proven.

## Decision

War will implement one local, in-process agent runtime with a small provider adapter boundary. The first
production adapter will call OpenAI Responses API directly using a user-owned API key stored in macOS
Keychain.

The Agent Runtime owns turns, context budgeting, tool sequencing, cancellation, and provider-neutral
events. The OpenAI adapter owns only transport and protocol translation. A separate Tool Gateway owns
permissions and execution.

The MVP supports one agent session per active application context. It does not support cloud execution,
multi-agent orchestration, automatic handoff, external agent harnesses, or generic OpenAI-compatible
endpoints.

## Alternatives Considered

### Reuse Warp hosted Agent Mode

This is the shortest path to a demo, but it retains account, server, entitlement, privacy, and product
dependencies that War explicitly rejects.

### Use ACP or external CLI agents first

This avoids implementing a model tool loop, but makes the native experience and permission guarantees
dependent on external agent behavior. ACP may be added later as an integration, not as the MVP runtime.

### Support OpenAI and Anthropic together

This validates provider neutrality earlier, but doubles protocol and testing work before the internal
event and tool contracts stabilize.

### Use a generic OpenAI-compatible adapter

Endpoint compatibility is inconsistent for tool calls, reasoning, continuation, and stream events. A
generic label would promise behavior War cannot validate across implementations.

## Consequences

### Positive

- War controls context selection, permissions, cancellation, and timeline semantics.
- The terminal has no dependency on a War or Warp inference service.
- Provider traffic is direct and auditable.
- One provider keeps the first vertical slice testable and bounded.

### Negative

- War must implement and maintain a safe agent loop.
- The first release serves only users with OpenAI API access.
- Provider protocol changes become War's maintenance responsibility.
- A poorly designed provider-neutral boundary could either leak OpenAI concepts or become premature
  abstraction.

### Follow-up

- Define the provider-neutral event model in the M3 feature spec.
- Store credentials in Keychain and prohibit plaintext fallback.
- Use deterministic sanitized stream fixtures in CI.
- Require a new spec and ADR before adding another production provider or changing permission defaults.

## Revisit When

Reconsider the provider sequence after M4 demonstrates the complete native timeline flow and there is
real demand for another provider or external agent protocol.
