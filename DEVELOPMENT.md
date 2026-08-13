# Development

This document defines the lightweight process used to keep War maintainable as a personal project.
It intentionally avoids team approval ceremonies while retaining reviewable decisions and reliable
quality gates.

## Working Model

- `main` is always expected to build and run.
- One roadmap milestone may be `Active` at a time.
- One primary vertical slice should be in progress at a time.
- Code changes go through a pull request, even when authored and reviewed by the same maintainer.
- Planning is milestone-based. Dates are not commitments during the pre-alpha phase.

## Branches and Commits

Use short-lived branch names:

- `feat/<topic>` for user-visible behavior
- `fix/<topic>` for defects
- `refactor/<topic>` for behavior-preserving structural work
- `docs/<topic>` for documentation-only work
- `chore/<topic>` for tooling and dependency maintenance

Use Conventional Commit subjects such as:

```text
feat: add local agent event stream
fix: preserve terminal output after agent cancellation
refactor: isolate provider transport from tool execution
docs: define M2 offline acceptance criteria
```

Keep commits focused. Preserve upstream commit authorship and message when cherry-picking an upstream
fix unless the patch must be materially rewritten.

## Change Classification

Routine changes do not require a spec. Examples include documentation corrections, dependency pins,
localized refactors, and reproducible bug fixes with an obvious expected behavior.

A product and technical spec are required when a change:

- introduces or changes user-visible behavior;
- adds an agent tool or changes tool approval behavior;
- changes data sent to a model or persisted locally;
- changes module boundaries, provider contracts, or terminal integration;
- introduces a migration, compatibility commitment, or new platform surface;
- is too broad to validate confidently in one pull request description.

An ADR is also required when a choice is durable, difficult to reverse, or constrains multiple future
features. See `docs/decisions/README.md`.

## Definition of Ready

A non-trivial implementation is ready to begin when:

- the user problem and desired outcome are explicit;
- goals and non-goals prevent obvious scope expansion;
- behavior is described as testable invariants;
- privacy, permission, cancellation, and failure paths are considered;
- affected boundaries and dependencies are identified;
- validation explains how success will be observed.

## Implementation Rules

- Deliver the smallest end-to-end slice that proves the behavior.
- Prefer removing an obsolete path over adding compatibility between old and new paths.
- Do not introduce a second UI, agent runtime, provider, or persistence path during the MVP.
- Add dependencies only when their value exceeds their build, security, and maintenance cost.
- Keep network access explicit and auditable.
- Keep model-provider details out of terminal and tool implementations.
- Every temporary feature flag must name its removal milestone in the pull request.

## Verification Commands

The imported baseline requires Git LFS objects and Yarn 4.0.1. Follow the clean-checkout setup and
known workarounds in [`docs/BASELINE.md`](docs/BASELINE.md). The standard verification commands are:

```bash
./script/bootstrap -y --skip-common-skills --skip-gcloud-auth
./script/format --check
./script/presubmit
cargo nextest run --no-fail-fast --workspace --exclude command-signatures-v2
cargo nextest run -p warp_completer --features v2
cargo test --doc
WARP_SKIP_COMMON_SKILLS_INSTALL=1 ./script/run
```

`script/presubmit` already includes the two nextest commands and doc tests; they are listed separately
to make individual failures reproducible. The imported workspace can generate more than 44 GiB of
build output during complete testing, so start with at least 60 GiB free. After the source tree is
simplified, these commands may be narrowed only if the replacement preserves equivalent coverage.

### Windows Development Host

Windows may be used for routine editing, compilation, Clippy, and unit tests to reduce load on the
macOS verification machine. This does not add a Windows release to the MVP or replace the required
macOS manual verification.

Use Git Bash for repository scripts and PowerShell 7 for PowerShell linting. The Windows environment
requires the pinned Rust toolchain, Git LFS, MSVC and the Windows SDK, CMake, Protobuf, LLVM with
`LIBCLANG_PATH` pointing to its `bin` directory, Node with Corepack, Yarn 4.0.1, `cargo-nextest`, and
`wgslfmt`. Enable Git long paths for the checkout. On a checkout that cannot materialize symlinks,
leave `core.symlinks=false` so Git uses its regular-file fallback rather than reporting a deleted
symlink.

The inherited Windows CI excludes `command-signatures-v2` and `warp_js`. Use the same exclusions
locally:

```bash
corepack install --global yarn@4.0.1
git config core.longpaths true
./script/format --check
./script/check_no_inline_test_modules
pwsh -NoProfile -Command "& './script/lint_powershell' -ci"
cargo clippy --locked --workspace --exclude warp_js --exclude command-signatures-v2 --all-targets --tests -- -D warnings
cargo clippy --locked -p warp --all-targets --tests -- -D warnings
cargo nextest run --locked --workspace --exclude command-signatures-v2 --exclude warp_js --no-run
cargo nextest run --locked --workspace --exclude command-signatures-v2 --exclude warp_js -E "not package(integration)" --no-fail-fast
cargo test --locked --workspace --exclude command-signatures-v2 --exclude warp_js --doc
```

The complete test build can exceed 50 GiB and has a high peak memory requirement. On a machine with
16 GiB of RAM, set `CARGO_BUILD_JOBS=1` for test compilation and keep a system-managed page file
enabled. Some inherited Windows history tests read the current user's PSReadLine history through the
Windows known-folder API; changing `APPDATA` alone does not isolate them. If those tests include the
developer's history, reproduce them under a clean Windows account or CI runner and record the local
result as an environment-isolation failure rather than passing evidence.

Windows validation does not cover macOS application bundling and signing, Keychain integration,
native window lifecycle, zsh and bash startup, or the M2 offline and network audit. Run those checks
on macOS before considering an applicable change complete.

## Testing Strategy

- Unit tests cover parsers, reducers, permission decisions, redaction, and provider event conversion.
- Contract tests replay recorded and sanitized provider streams without network access.
- Integration tests cover PTY behavior, tool execution, cancellation, and persistence boundaries.
- GUI tests cover high-value timeline and approval flows.
- Manual macOS verification covers shell startup, command blocks, alternate screen applications,
  keyboard input, and the changed user flow.
- Bug fixes include a regression test unless the behavior cannot be automated; exceptions are
  explained in the pull request.

Provider tests must not use live API keys in CI. Fixtures must not contain real prompts, repository
content, credentials, or personal data.

## Definition of Done

A change is complete when:

- accepted spec invariants are satisfied;
- formatting and linting pass;
- relevant automated tests pass;
- applicable macOS manual verification is recorded;
- user-visible changes include a screenshot or short recording when useful;
- privacy and permission impacts are reviewed;
- logs and fixtures contain no secrets;
- documentation reflects the resulting behavior;
- no unrelated cleanup or indefinite feature flag remains;
- untested behavior and residual risks are stated in the pull request.

## Upstream Changes

War tracks a fixed Warp baseline rather than continuously merging upstream.

1. Use the baseline commit recorded in `docs/BASELINE.md`.
2. Fetch upstream changes without merging them into `main`.
3. Select only relevant PTY, rendering, shell, platform, and security fixes.
4. Cherry-pick fixes individually and run War's full verification.
5. Resolve conflicts in favor of War's product and architecture contracts.
6. Record material divergence or adaptation in the pull request.

## Roadmap Maintenance

Milestones move through `Planned`, `Active`, `Blocked`, and `Done`. A milestone becomes `Done` only
after every exit criterion is demonstrated. New ideas that do not belong to the active milestone go
to the candidate list rather than entering implementation immediately.
