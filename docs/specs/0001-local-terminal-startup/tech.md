# Technical Spec: Local Terminal Startup

**Status:** Accepted
**Owner:** War maintainer
**Created:** 2026-08-12
**Accepted:** 2026-08-12
**Product spec:** [product.md](product.md)

## Context

The OSS entry point in `app/src/bin/oss.rs` identifies the product as `WarpOss` and supplies production
Warp Server and Oz configuration. `initialize_app` in `app/src/lib.rs` loads persisted Warp
authentication and eagerly registers server, authentication, telemetry, cloud synchronization, billing,
updater, hosted-agent, and remote-execution models before a terminal window is opened. It also starts
the inherited loopback HTTP server for ordinary GUI launches.

`RootView::new` selects between authentication, onboarding, and a workspace using hosted authentication
state. `Workspace`, `PaneGroup`, `TerminalView`, and `Input` then eagerly construct account, Drive,
sharing, hosted-agent, and server-backed UI even when the selected input mode is Shell. Omitting only the
top-level registrations would therefore panic on missing singleton models.

The application identifier also namespaces Application Support, UserDefaults, SQLite, and Keychain
state, while the OSS user configuration path is independently fixed to `.warp-oss`. The identity and
startup composition must change together to prevent War from reading inherited Warp state.

This work implements the accepted boundaries in `docs/PRODUCT.md`, `docs/ARCHITECTURE.md`, ADR 0001,
and M2 of `docs/ROADMAP.md`. It does not introduce a new architecture decision.

## Proposed Changes

### Canonical identity

- Keep the internal package/library name `warp` and `Channel::Oss` where they are not externally visible.
- Rename the OSS GUI binary to `war` and its bundle to `War.app`.
- Set the runtime and bundle application identifier to `tech.oaix.War`, URI scheme to `war`, log filename
  to `war.log`, visible copyright to `Copyright © 2026 oaix`, and product descriptions to War wording.
- Resolve OSS user-global configuration under `~/.war` and bundle-derived state under
  `~/Library/Application Support/tech.oaix.War`.
- Remove use of the inherited Warp application-group entitlement and container.
- Make the standard run and bundle scripts deterministically build the War OSS channel rather than probe
  private channel configuration.
- Replace the inherited icon and installer branding with a neutral War asset. Keep upstream license and
  source attribution unchanged.
- Do not scan old directories or implement a compatibility fallback.

### Local application composition

Refactor the native GUI composition root so a normal War launch registers only local settings,
rendering, persistence, history, PTY, shell, workspace, tab, pane, and terminal responsibilities. Do not
construct or register hosted service clients, cloud models, telemetry, crash upload, changelog, updater,
IAP, remote execution, installation detection, or local-control servers.

Lifecycle callbacks retain local persistence and process cleanup but remove authentication accounting,
telemetry recording and flushing, and update installation. Retained product-telemetry macros become
inert on the War path until their call sites are deleted; a fake server or fake identity is not used.

### Local window and terminal construction

- Make the native `RootView` own a local `Workspace` directly, without auth/onboarding state or hosted
  subscriptions.
- Make account, Drive, sharing, billing, code-review, updater, and hosted-agent views absent or lazy so
  construction of a local workspace does not require their singleton models.
- Remove `ServerApi` from the resource chain passed through `Workspace`, `PaneGroup`, `TerminalView`, and
  `Input` for a local shell.
- Construct shell-only terminal input with local editing, history, completion, command execution, and
  PTY wiring. Agent UI will be attached through the M3/M4 runtime rather than the inherited hosted stack.
- Do not create API-key, remote-server, sharing, network-status, or auth-telemetry controllers for an
  ordinary local PTY.
- Prefer deletion or lazy construction over no-op hosted implementations.

### Local restoration

Continue reading War's own local application persistence. Restore only state that can be represented by
local terminal windows, tabs, panes, and working directories without initializing an excluded service.
If validation or decoding fails, open one new local terminal and leave the unreadable persisted data
untouched. No Warp data migration or pre-alpha format compatibility layer is added.

Physical deletion of every now-unreachable cloud module and dependency continues through M2 and M5;
the first slice requires structural absence from startup rather than complete source deletion.

## Data Flow

```text
war entry point
  -> set War identity
  -> initialize local settings and persistence
  -> initialize renderer and PTY infrastructure
  -> validate War-local restored terminal state
  -> open restored local workspace or one new terminal
  -> resolve zsh/bash and spawn local PTY
  -> render shell input, output, and command blocks
```

No branch in this startup flow reaches an authentication, provider, telemetry, updater, cloud, or
loopback HTTP boundary.

## Interfaces and State

The canonical external identifiers are constants or channel configuration values used consistently by
the entry point, Cargo bundle metadata, core path resolution, URI handling, and macOS scripts. The
existing three-component `AppId` supports `tech.oaix.War` without a parser change.

The local initializer should expose a small, testable startup policy rather than infer local behavior
from a collection of feature flags. Temporary compile-time gates are permitted only when their owner,
purpose, and M5 removal are explicit; no permanent cloud/local product matrix is introduced.

Persisted terminal records retain their existing local schema for this slice. Restoration adds a
validation boundary that rejects variants requiring excluded services. Pre-alpha data has no
compatibility guarantee.

## Security and Privacy

- No provider boundary exists in M2, and no model request is made.
- The secure-storage service uses the War data domain. It must not query a Warp service name; API-key
  storage is introduced separately in M3.
- No account identifier, anonymous Firebase identifier, telemetry context, or persisted Warp credential
  is created or loaded.
- Local logs remain diagnostic only and omit sensitive values and terminal content by default.
- The inherited in-memory network logger is not registered because it can capture sensitive request
  bodies and no hosted request is valid on this path.
- Shell commands remain explicit user input. Agent permission and cancellation rules are unchanged.
- Runtime network auditing includes DNS, sockets, WebSockets, HTTP, loopback listeners, and child-process
  attribution; an internal HTTP hook is not accepted as complete evidence.

## Alternatives Considered

### Only skip onboarding and login

This leaves production clients, persisted Warp authentication, telemetry, cloud synchronization, and
the loopback server active. It changes visible UI without satisfying the offline or privacy boundary.

### Configure invalid or local server URLs

Black-hole endpoints retain hidden dependencies, retries, delays, and a path for future regressions.
Excluded services must not be constructed.

### Register no-op cloud and authentication models

This would preserve the inherited dependency graph and create a parallel fake product architecture.
Local terminal constructors should stop depending on excluded responsibilities instead.

### Delete every excluded module in one change

That would be difficult to review and validate while preserving terminal behavior. The first slice
removes excluded services from the startup dependency closure; focused physical deletion follows.

## Risks and Mitigations

| Risk | Mitigation |
| --- | --- |
| Hidden singleton access panics after cloud registrations are removed | Add a startup test that registers only the local dependency closure and opens a real workspace and terminal |
| Shell input construction still instantiates hosted Agent state | Split shell-only construction and require excluded models to be absent in the dependency-closure test |
| Restored cloud records activate removed services | Validate restored pane variants before construction and fall back to a fresh local shell |
| Bundle metadata and runtime identity disagree | Add one macOS bundle-identity assertion script used by run and bundle verification |
| War silently reads old Warp paths | Add exact path tests and a macOS sentinel-data audit; do not add compatibility fallbacks |
| Internal request hooks miss sockets or child traffic | Pair automated structural tests with an OS-level, process-attributed macOS network audit |
| Large refactor regresses mature terminal behavior | Preserve focused workspace, pane, PTY, shell, and integration tests and verify zsh, bash, tabs, command blocks, and alternate screen manually |
| Neutral temporary artwork is mistaken for final release design | Record it as M2 branding, with replacement allowed before M6 without changing identity or behavior |

## Testing and Validation

| Product invariant | Verification |
| --- | --- |
| 1 | Unit dependency-closure test plus GUI integration test from isolated state |
| 2 | zsh and bash integration tests and offline macOS launch |
| 3 | Assert excluded singleton models are absent; inspect startup composition |
| 4 | Assert no local server model/listener and perform macOS process-level network audit |
| 5 | Core identity/path tests and macOS bundle identity script |
| 6 | Exact path tests plus sentinel Warp data, UserDefaults, app-group, and Keychain audit |
| 7 | Local restoration tests for valid, missing, corrupt, and excluded pane state |
| 8 | Telemetry registration/persistence tests and local log review |
| 9 | Existing terminal tests plus manual command block, tab, history, search, copy, and alternate-screen checks |
| 10 | Bundle plist/icon/menu inspection and license-file verification |

Run focused tests first, followed by the exact applicable commands in `DEVELOPMENT.md`:

```bash
cargo nextest run -p warp_core
cargo run -p integration --bin integration -- test_local_offline_startup
./script/format --check
./script/presubmit
cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2
cargo nextest run -p warp_completer --features v2
cargo test --doc
WARP_SKIP_COMMON_SKILLS_INSTALL=1 ./script/run --dont-open
```

On Windows, use the documented exclusions and treat compilation, Clippy, and unit tests as preliminary
evidence only. Required macOS verification builds and signs `War.app`, tests zsh and bash, validates
session restoration and alternate-screen behavior, launches with networking disabled, and captures an
online startup network audit with process attribution. Cloud verification is run on one representative
macOS runner only after cheaper local gates pass; the change is OS-sensitive but not
architecture-sensitive.

## Rollout and Removal

Direct replacement. There is no migration from Warp identities or persisted data. Any temporary source
gates used to isolate still-compiled inherited modules are owned by M2 and removed no later than M5.

## Follow-ups

- Rename externally visible inherited shell-integration protocol identifiers as one coordinated M2
  terminal-integration change after zsh and bash startup are stable.
- Delete unreachable hosted authentication, Drive, sharing, billing, Oz, cloud-agent, updater, telemetry,
  editor, and collaboration modules in focused M2/M5 changes.
- Replace the neutral M2 icon with final artwork before M6 if desired.
