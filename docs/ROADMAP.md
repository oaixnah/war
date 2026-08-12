# Roadmap

**Planning model:** Milestones with acceptance criteria
**Active milestone:** M1 Upstream Baseline
**Last reviewed:** 2026-08-12

## Operating Rules

- Milestones use `Planned`, `Active`, `Blocked`, or `Done`.
- Only one milestone may be `Active`.
- A milestone is `Done` only when every exit criterion is demonstrated.
- Dates are forecasts only and are omitted until the baseline is understood.
- Work outside the active milestone goes to Candidates instead of entering implementation.
- Scope changes update `docs/PRODUCT.md` before they update this roadmap.

## Milestone Summary

| Milestone | Status | Outcome |
| --- | --- | --- |
| M0 Foundation | Done | Repository rules, plans, templates, and legal intent are explicit |
| M1 Upstream Baseline | Active | A reproducible, recorded Warp source baseline builds on macOS |
| M2 Local Terminal | Planned | War is rebranded, account-free, offline, and independent of Warp services |
| M3 Agent Core | Planned | A headless local agent runtime completes tool loops through OpenAI Responses |
| M4 Native Timeline | Planned | Shell and agent interactions work safely in one terminal timeline |
| M5 Simplification | Planned | Excluded product surfaces and dependencies are removed from the default build |
| M6 Alpha | Planned | A documented macOS application is suitable for daily dogfooding |

## M0 Foundation

### Scope

- Define the product contract and MVP non-goals.
- Define target architecture, privacy boundaries, and tool permissions.
- Define development workflow, spec threshold, ADR process, and definition of done.
- Establish the milestone roadmap and pull request checklist.
- Record attribution, license intent, and trademark separation.
- Define the upstream history migration strategy.

### Exit Criteria

- Root documentation links to every authoritative document.
- Product, architecture, development, and roadmap documents use consistent terminology.
- Spec and ADR templates are ready for the first implementation change.
- The upstream history strategy is accepted.
- No placeholder application implementation is introduced.

## M1 Upstream Baseline

### Scope

- Tag the current pre-upstream history.
- Add `warpdotdev/warp` as the upstream remote and select an exact `master` commit.
- Rebuild `main` from that commit and replay War's foundation documents.
- Retain upstream AGPL and MIT license texts and copyright notices.
- Run the upstream macOS bootstrap, build, tests, and `warp-oss` application.
- Inventory workspace crates, default features, startup services, network endpoints, and build issues.
- Record commands, commit SHA, toolchain, baseline metrics, and known failures in `docs/BASELINE.md`.

### Exit Criteria

- A clean macOS checkout can follow documented steps to build and launch the baseline.
- The exact upstream commit and license files are present.
- Baseline failures are documented rather than hidden or patched without explanation.
- No War product behavior is implemented in the baseline commit.
- Rewriting remote `main` receives explicit confirmation immediately before push.

### Evidence

- The exact baseline, inventory, build results, runtime observations, and known failures are recorded in
  [`BASELINE.md`](BASELINE.md).
- The baseline builds, signs, and launches as `WarpOss.app`; skipping inherited login starts a local zsh
  session.
- M1 remains `Active` until the validated migration branch replaces `main` with explicit confirmation.

## M2 Local Terminal

### Scope

- Rebrand the application, executable, bundle identifier, URI scheme, icons, and data paths.
- Create a local-only startup path with no login or conversion flow.
- Remove startup dependencies on Warp Server, Oz, Firebase, GraphQL, billing, and telemetry.
- Retain local shell, terminal blocks, command input, tabs, history, and alternate-screen behavior.
- Begin deleting unreachable cloud and collaboration modules rather than only hiding their UI.
- Audit runtime network activity.

### Exit Criteria

- War launches directly into a local shell without an account.
- War remains a useful terminal with networking disabled.
- zsh, bash, command blocks, tabs, and alternate-screen applications pass manual checks.
- A runtime audit observes no Warp, authentication, telemetry, or cloud agent traffic.
- Application metadata and visible branding no longer imply affiliation with Warp.

## M3 Agent Core

### Scope

- Define the provider-neutral agent event and conversation models.
- Implement OpenAI Responses streaming behind a provider adapter.
- Store the user API key in macOS Keychain.
- Implement cancellation, bounded retries, context limits, and structured provider errors.
- Implement the tool gateway for read, glob, grep, shell, and patch operations.
- Implement approval states and repository path boundaries.
- Add deterministic provider fixtures and tool security tests.

### Exit Criteria

- A headless test completes a multi-turn tool loop from prompt to final response.
- No live key or network request is required by tests.
- Provider code cannot execute tools directly.
- Shell and patch tools cannot start without an approval token.
- Cancellation stops provider streaming and active tool work cleanly.
- Keychain and log handling pass a manual secret review.

## M4 Native Timeline

### Scope

- Add explicit Shell and Agent input modes.
- Render prompts, streamed messages, tool requests, approvals, results, diffs, errors, and cancellation
  in the terminal timeline.
- Allow explicit attachment of command blocks and selections.
- Add only the low-risk automatic project metadata defined by the architecture.
- Persist local conversations with deletion controls.
- Preserve normal terminal interaction while an agent turn is active or cancelled.

### Exit Criteria

- A user can complete a small repository change without opening a separate chat panel.
- Every agent-requested shell command and patch has a visible approve or reject step.
- Attached and automatically supplied context are distinguishable.
- Switching back to Shell mode does not lose terminal control.
- Errors and cancellation leave the session in a usable state.
- High-value flows have automated integration coverage and manual visual evidence.

## M5 Simplification

### Scope

- Remove Auth, Drive, team, sharing, billing, Oz, cloud runner, orchestration, notebook, voice,
  computer-use, embedded editor, LSP, and code-review surfaces from the default product.
- Remove server-oriented crates and dependencies no longer required by retained code.
- Collapse temporary migration flags and delete dead branches.
- Re-evaluate workspace members, default features, binary size, startup work, and build time.
- Preserve terminal behavior while extracting neutral types from excluded modules.

### Exit Criteria

- Excluded product surfaces are absent from the default build and navigation.
- No retained module depends on Warp account, entitlement, billing, or cloud-agent types.
- Every remaining default feature represents a shipped War capability.
- Workspace and dependency audits show documented reduction from the M1 baseline.
- Terminal and agent regression suites remain green.

## M6 Alpha

### Scope

- Produce a repeatable macOS application bundle.
- Document installation, configuration, privacy behavior, limitations, and source availability.
- Complete license, notice, and third-party dependency review.
- Run sustained daily dogfooding and fix data-loss, terminal-compatibility, and permission defects.
- Define the support boundary for the first public pre-release.

### Exit Criteria

- A clean machine can install, configure, and run War from documented instructions.
- Offline terminal and AI-native task acceptance flows pass end to end.
- Known limitations and persistence compatibility are explicit.
- No known critical secret exposure, unapproved side effect, terminal corruption, or data-loss issue
  remains open.
- The source corresponding to the distributed binary is available under the applicable licenses.

## Candidates

Candidates are not commitments and must not enter MVP implementation without a product scope change:

- Linux support
- Anthropic provider adapter
- configurable read-only command approval profiles
- MCP tools
- session export
- signed and notarized release automation
