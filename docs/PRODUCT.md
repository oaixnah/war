# Product Contract

**Status:** Accepted
**Last reviewed:** 2026-08-11

## Vision

War is a local-first AI-native terminal for developers who want an agent integrated into their shell
workflow without adopting a cloud development platform or a terminal-shaped IDE.

War is derived from Warp because Warp already solves difficult terminal problems: PTY management,
GPU rendering, structured command blocks, modern input, and native application integration. War uses
that foundation to build a deliberately smaller product.

## Target User

The initial user is a macOS developer who:

- works primarily from a terminal;
- wants a coding agent that understands terminal context;
- owns and controls their model API credentials;
- expects the terminal to work offline and without login;
- values visible approval boundaries over autonomous background execution;
- does not need cloud agents, team collaboration, or an embedded IDE.

## Product Principles

### Local terminal first

Starting a shell, running commands, using alternate-screen applications, and reviewing command
history must never require an account or network connection.

### One timeline

Shell commands, command output, user prompts, agent messages, tool calls, approvals, diffs, and tool
results are represented in one ordered timeline. AI is not a detached chat sidebar.

### Explicit intent

Users explicitly select Shell or Agent input. War does not automatically classify natural language,
because a classification error can execute unintended shell input.

### User-owned inference

The user supplies the model API key. War stores it using the macOS Keychain and sends model requests
directly to the configured provider. War does not proxy inference through a War or Warp service.

### Visible side effects

The agent may reason and request tools, but shell commands and file mutations require approval in the
MVP. The interface shows the exact proposed operation and its result.

### Deliberate scope

War optimizes for a reliable terminal and one effective local agent. A feature is not justified merely
because Warp or another development environment includes it.

## MVP Capabilities

### Terminal

- macOS application
- local zsh and bash sessions
- command blocks with command, output, exit status, and working directory
- command editing and history
- tabs
- search, copy, and command re-entry
- alternate-screen application support

### Agent

- explicit Agent input mode
- OpenAI Responses API using a user-provided key
- streaming text, reasoning status, and tool calls
- explicit attachment of a command block or text selection
- safe automatic project metadata such as working directory and Git branch
- file read, glob, grep, shell command, and patch tools
- approval, rejection, cancellation, and visible errors
- local conversation persistence with documented retention and deletion behavior

### Privacy and safety

- no product telemetry
- no Warp account or War account
- no implicit upload of complete terminal history or environment variables
- secret redaction before provider requests and before persistence
- visible network boundary for model traffic
- no live credentials in logs, tests, or crash data

## Non-goals

The MVP does not include:

- Windows or Linux releases;
- multiple simultaneous agents or agent orchestration;
- cloud agents, remote runners, scheduling, or handoff;
- team accounts, sharing, synchronization, or billing;
- Warp Drive, notebooks, or workflow marketplaces;
- an embedded file tree, code editor, LSP client, or code review product;
- voice input or computer use;
- automatic natural-language detection;
- Anthropic, Google, xAI, or generic OpenAI-compatible endpoints;
- a plugin ecosystem;
- compatibility guarantees for pre-alpha persisted data.

## Core User Flow

1. The user starts War without logging in and receives a local shell.
2. Commands and output appear as structured terminal blocks.
3. The user explicitly switches to Agent mode and describes a coding task.
4. The user optionally attaches a relevant block or selection.
5. War sends only the declared context to the configured OpenAI endpoint.
6. The agent streams progress into the same timeline.
7. Read-only project tools may complete automatically.
8. Shell commands and file changes wait for user approval.
9. Results, diffs, failures, and cancellation remain visible in the timeline.
10. The user can return to Shell mode at any time.

## MVP Success Criteria

The MVP is successful when a clean macOS installation can:

- launch and run a useful terminal with networking disabled;
- complete normal zsh work and alternate-screen workflows without an account;
- accept an OpenAI API key without storing it in plaintext configuration;
- use selected terminal context to complete a small repository change;
- show and enforce approval for every agent-requested side effect;
- cancel an in-flight model response or tool operation without corrupting the terminal session;
- demonstrate that no Warp service or undeclared telemetry endpoint is contacted.

## Scope Change Rule

A proposal that expands a non-goal requires a new product spec and explicit update to this contract.
Implementation convenience alone is not sufficient reason to expand the MVP.
