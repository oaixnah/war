# Upstream Baseline

**Status:** Recorded
**Baseline date:** 2026-08-12
**Upstream repository:** <https://github.com/warpdotdev/warp>

## Provenance

War is based on this exact Warp commit:

```text
b0c7a7674bc4d779566d9b67db8573cad03e9c9d
```

The commit was authored on 2026-08-11 with the subject:

```text
Log process-group cancellation outcomes for diagnosability (CSAT-10070 / GH#13852) (#14937)
```

The pre-import War history is preserved by the annotated tag `pre-upstream-foundation`, which resolves
to `7863bfd6fc39e8f564cb8e6ef49ae00b1924fc15`. War's foundation documents were replayed on top of the
upstream commit without modifying `app/`, `crates/`, Cargo manifests, or `Cargo.lock`.

The upstream license files are retained unchanged:

| File | SHA-256 |
| --- | --- |
| `LICENSE-AGPL` | `16f8dac5fd5755d74ba05cb377c23f49e794252d200548fb3b227e1c5dcbfaec` |
| `LICENSE-MIT` | `0cb5f8a282734dd41844fc3bdc28bda274cc71db8de289a9959263b61210daa9` |

## Reproducing The Baseline

The upstream repository uses Git LFS. At the time of validation, War's GitHub LFS endpoint could not
serve the inherited objects, so a clean checkout must obtain them from the recorded upstream remote:

```bash
GIT_LFS_SKIP_SMUDGE=1 git clone git@github.com:oaixnah/war.git
cd war
git remote add upstream https://github.com/warpdotdev/warp.git
git lfs pull upstream
```

Seven LFS objects are required: four Windows PDB files and three ONNX input-classifier models. The
macOS GUI build uses the ONNX models. Running `git lfs pull upstream` materialized all seven objects
during validation.

The command-signatures build requires Yarn 4.0.1. Homebrew Node 26 did not include Corepack, and the
globally installed Yarn 1.22.22 was incompatible. This temporary setup reproduced the required Yarn
version without replacing an existing global pnpm installation:

```bash
COREPACK_ROOT="$(mktemp -d)"
npm install --prefix "$COREPACK_ROOT" corepack@0.34.6
"$COREPACK_ROOT/node_modules/.bin/corepack" install --global yarn@4.0.1
export PATH="$COREPACK_ROOT/node_modules/.bin:$PATH"
yarn --version
```

Bootstrap and build commands:

```bash
./script/bootstrap -y --skip-common-skills --skip-gcloud-auth
./script/format --check
./script/presubmit
WARP_SKIP_COMMON_SKILLS_INSTALL=1 ./script/run --dont-open
open target/debug/bundle/osx/WarpOss.app
```

Use at least 60 GiB of free disk space before running the complete presubmit suite. This validation
started with less space and the workspace test-link phase exhausted the disk after generating tens of
gigabytes of build output.

## Validation Environment

| Component | Version |
| --- | --- |
| macOS | 15.7.9 (24G830), arm64 |
| Xcode | 26.3 (17C529) |
| Rust | 1.92.0 (`ded5c06cf`) |
| Cargo | 1.92.0 (`344c4567c`) |
| Git | 2.50.1 (Apple Git-155) |
| Git LFS | 3.7.1 |
| Node | 26.5.0 |
| Yarn | 4.0.1 through Corepack 0.34.6 |
| PowerShell | 7.6.4 |
| PSScriptAnalyzer | 1.25.0 |
| cargo-nextest | 0.9.143 |
| cargo-bundle | 0.11.0 |

## Workspace Inventory

`cargo metadata --locked --offline --no-deps --format-version 1` succeeded and reported:

| Metric | Count |
| --- | ---: |
| Workspace packages | 79 |
| Default workspace members | 11 |
| Binary targets | 19 |
| App feature keys exposed by Cargo metadata | 304 |
| Explicit app `[features]` entries | 303 |
| App default features | 197 |
| Direct Git dependency entries | 26 across 20 repositories |
| Locked Git packages | 37 across 21 repositories |

The workspace members are selected by `crates/*` plus `app`. The 11 default members are `warp`,
`channel_versions`, `command`, `warp_editor`, `warp_graphql`, `markdown_parser`, `sum_tree`, `warpui`,
`warp_completer`, `warp_terminal`, and `warp_util`. The comment in the root manifest says only
`serve-wasm` and `integration` are omitted, but the explicit default list omits 66 additional packages.

The principal application entry points are:

| Surface | Package | Default binary |
| --- | --- | --- |
| macOS GUI | `warp` | `warp-oss` |
| Headless TUI | `warp_tui` | `warp-tui-oss` |

The TUI is a workspace member but not a default member. Both entry points use production Warp Server
and Oz configuration and converge on the shared application initialization path.

## Inherited Product Surface

The imported baseline includes far more than War's target product. Notable workspace crates and app
modules include:

- authentication, onboarding, Firebase, Warp Server clients, GraphQL, and session sharing;
- Drive, cloud objects, synchronization, teams, billing, and managed secrets;
- hosted Agent Mode, Oz, multi-agent orchestration, cloud runners, and remote execution;
- MCP, computer use, voice input, natural-language detection, and cloud codebase indexing;
- an embedded editor, file tree, notebooks, LSP, code review, and language tooling;
- telemetry, tracing, Sentry integration, feature rollout, and autoupdate infrastructure.

Default app features enable many of these surfaces, including Agent Mode, cloud mode, orchestration,
Drive context, session sharing, billing, editor and file-tree features, MCP, computer use, code review,
cloud runners, and global AI analytics collection.

Shared startup initialization registers server API and authentication providers, telemetry collection,
cloud models and sync queues, team and update managers, editor and language-server state, and agent,
orchestration, and remote-server models. This is an inherited architecture inventory, not the target
dependency direction in [`ARCHITECTURE.md`](ARCHITECTURE.md).

## Network Inventory

Static inspection found these inherited network boundaries:

| Boundary | Baseline behavior |
| --- | --- |
| Warp Server | New-user model queries, GraphQL, login, Drive, AI, cloud-agent, and update APIs |
| Warp RTC | GraphQL WebSocket for signed-in team or Drive state |
| Session sharing | WebSocket opened when creating or joining a shared session |
| Firebase | Token refresh and custom-token exchange for stored accounts |
| Oz | Hosted agent and cloud execution configuration |
| Common skills | `script/run` downloads and executes a resolver from a mutable remote ref by default |
| Channel config | `script/run` probes a private GitHub repository before selecting `warp-oss` |
| Local HTTP | GUI listens on `127.0.0.1:9282` for installation detection |

The inherited agent does not satisfy War's direct-provider requirement. It sends model requests,
including BYOK configuration and selected context, through the Warp Server multi-agent endpoint.

During manual startup, `warp-oss` immediately established a connection through the machine's local
network proxy and listened on `127.0.0.1:9282`. Static inspection shows that a new installation requests
the public model list from Warp Server during onboarding. Runtime endpoint attribution beyond the local
proxy was not attempted in M1; M2 requires a dedicated network audit.

The OSS channel has no effective RudderStack destination or Sentry configuration, but telemetry remains
enabled internally. On application exit, the baseline wrote queued telemetry events to local disk. The
privacy settings UI hides the telemetry control when no telemetry configuration exists.

The default in-memory network log captures request debug data and supported request bodies without an
identified redaction layer. Those bodies may include refresh tokens, prompts, model API keys, and
attached context. No credentials were supplied during M1 validation.

## Verification Results

| Check | Result |
| --- | --- |
| `git lfs pull upstream` | Passed; all seven objects materialized |
| `./script/bootstrap -y --skip-common-skills --skip-gcloud-auth` | Top-level command completed with known nested failures and warnings |
| `./script/format --check` | Passed in 14 seconds |
| `./script/check_no_inline_test_modules` | Passed |
| Workspace Clippy command from `script/presubmit` | Passed with warnings denied |
| Default GUI Clippy command from `script/presubmit` | Passed with warnings denied |
| `warp_completer` Clippy command from `script/presubmit` | Passed with warnings denied |
| C, C++, and Objective-C clang-format check | Passed |
| WGSL formatting check | Passed |
| `./script/lint_powershell -ci` | Passed; 19 information-level findings |
| Full workspace nextest command | Did not complete; linker failed with `errno=28` after disk exhaustion |
| `cargo nextest run -p warp_terminal` | Passed: 115 tests, 2 skipped |
| `cargo nextest run -p warp_completer --features v2` | Passed: 123 tests, 4 skipped |
| `cargo test --doc` | Passed: 43 tests, 4 ignored |
| Markdown lint | Passed with zero errors |
| `WARP_SKIP_COMMON_SKILLS_INSTALL=1 ./script/run --dont-open` | Passed in 8 minutes 26 seconds |
| Bundle code-signature verification | Passed |
| Manual `warp-oss` launch and zsh startup | Passed after completing onboarding and explicitly skipping login |

The successful debug build produced an arm64 `WarpOss.app` and DMG:

| Artifact | Size | SHA-256 |
| --- | ---: | --- |
| `WarpOss.app` | 774 MiB | Binary: `90a3a64786dfd356016094b4cdb298afd87c58b0b21254cc08ff46d35e80c214` |
| `WarpOss.dmg` | 143 MiB | `f5d58e740383bb2446b1750241404a432bfbd6f9a1b97ce497de43d850981068` |

The bundle retains upstream identity: bundle identifier `dev.warp.WarpOss`, URI scheme `warposs`, Warp
name and icon, Denver Technologies copyright, and editor document associations. Rebranding belongs to
M2.

Manual startup entered multi-step onboarding and a login screen. Selecting the explicit skip action
then started `/bin/zsh` 5.9, completed Warp shell integration bootstrap, loaded local history, and
created a usable terminal model. The app and terminal-server processes shut down cleanly. This proves
the upstream terminal can run; it does not satisfy War's requirement to launch directly into a shell.

## Known Baseline Failures

### Git LFS

The War origin returned `This repository exceeded its LFS budget` during checkout and bootstrap. Fetching
the recorded objects from `upstream` succeeded. Until War's LFS hosting is fixed, clean checkouts must use
the reproduction sequence above.

### Bootstrap reproducibility

On a machine without PowerShell, the script runs `brew install --cask powershell`, but current Homebrew
provides stable PowerShell as a formula and only a preview cask. Installing the stable formula allowed
bootstrap to continue.

`cargo-binstall` repeatedly received GitHub API 403 responses while resolving `diesel_cli`, fell back to
source, and failed because the installed MySQL client 9.7.1 is newer than `mysqlclient-sys` supports. A
later fallback installed an x86_64 prebuilt binary. The app build does not require the Diesel CLI.

Bootstrap and run scripts also probe the private `warp-channel-config` repository. Lack of access is
reported and the scripts continue with the OSS channel.

### JavaScript toolchain

The source requires Yarn 4.0.1, but bootstrap does not install or enable a matching Yarn. The Rust build
fails in `command-signatures-v2/build.rs` when Yarn 1 is first on `PATH`. The reproduction instructions
pin Corepack and Yarn explicitly.

### Full test disk usage

The complete nextest command exhausted local disk during test-binary linking. The linker reported
`ld: write() failed, errno=28`; no test assertion ran or failed. `cargo clean` removed 44.8 GiB of
generated output. Core terminal, completer, doc tests, formatting, and all Clippy configurations passed.

### Product and privacy violations

The baseline does not launch directly into a local terminal, is not offline, retains Warp branding and
service dependencies, and persists local telemetry. Its hosted agent and network logging do not satisfy
War's provider and secret-handling boundaries. These are expected M2 through M5 removals and must not be
used as foundations for new War behavior.

### Process-group cancellation risk

The selected upstream commit adds logging and rejects process-group IDs below 2, but explicitly leaves a
PID-reuse window in command cancellation unresolved. A stale registered process-group ID could target an
unrelated recycled process group. This inherited risk must be reviewed before War is used as a daily
terminal.
