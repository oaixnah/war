# ADR 0001: Derive War from a Fixed Warp Client Baseline

**Status:** Accepted
**Date:** 2026-08-11
**Decision owners:** War maintainer

## Context

Building a terminal emulator, PTY integration, GPU renderer, command block model, input editor, and
native macOS application from scratch would consume most of the project's effort before validating the
AI-native product.

Warp has released its client under AGPL-3.0-only, with `warpui_core` and `warpui` under MIT. The client
contains the terminal foundation War needs, but it also contains a large product and dependency graph
for hosted authentication, Warp Server, Drive, billing, cloud agents, orchestration, collaboration, and
IDE features.

A continuously merged fork would repeatedly reintroduce excluded product assumptions and make local
simplification dependent on upstream's roadmap.

## Decision

War will derive from one exact commit on Warp's `master` branch and preserve the upstream Git history.
The baseline commit, toolchain, build instructions, licenses, and known failures will be recorded during
M1.

War will not regularly merge upstream `master`. Relevant PTY, renderer, shell, platform, and security
fixes will be reviewed and cherry-picked individually with original attribution.

Before rebuilding `main`, the current pre-upstream history will be tagged. War's foundation documents
will be replayed as commits on top of the selected upstream baseline. Rewriting the remote branch will
require explicit confirmation immediately before push.

## Alternatives Considered

### Build a new terminal from scratch

This would provide the cleanest architecture and permissive licensing choices, but it delays validation
of the actual product and duplicates difficult terminal work that Warp already makes available.

### Continuously merge Warp upstream

This would provide more upstream fixes automatically, but it makes War repeatedly reconcile removed
cloud and product surfaces. Merge convenience would pressure War to retain the complexity it exists to
remove.

### Import a source snapshot without history

This produces a simple Git graph, but weakens attribution, source archaeology, and selective
cherry-picking of later fixes.

## Consequences

### Positive

- War can validate its product using a mature terminal foundation.
- Upstream code provenance and commit-level context remain available.
- War controls when upstream complexity enters the project.
- Security and platform fixes can still be selected when relevant.

### Negative

- Most of the client remains AGPL-3.0-only.
- Selective updates require manual review and conflict resolution.
- War becomes responsible for fixes after substantial divergence.
- The first import and rebrand will be a large repository transition.

### Follow-up

- Record the exact baseline and dependency inventory in `docs/BASELINE.md` during M1.
- Retain upstream copyright and license notices.
- Remove Warp names, icons, identifiers, and implied affiliation before distributing War binaries.
- Keep terminal behavior tests strong enough to validate cherry-picked changes independently.

## Revisit When

Reconsider this strategy if the retained Warp terminal foundation cannot be separated from excluded
services, upstream licensing changes incompatibly, or maintaining the fork costs more than replacing
the terminal substrate.
