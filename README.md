# War

War is an experimental, local-first AI-native terminal derived from the open-source Warp client.
It aims to keep the terminal, command blocks, and native agent interaction while removing cloud,
account, collaboration, and IDE product surfaces.

> [!WARNING]
> War currently contains an unmodified Warp baseline, not a usable War release. It still uses Warp
> onboarding, branding, services, and local telemetry; do not supply credentials or sensitive context.

## Principles

- The terminal works without an account and remains useful offline.
- Shell and agent activity share one structured timeline.
- Agent input is explicit; War does not guess whether text is a command or a prompt.
- Model credentials belong to the user and stay in the macOS Keychain.
- Tool side effects pass through a visible permission boundary.
- Product scope is deliberately smaller than Warp.

## Project Status

The active milestone is **M1: Upstream Baseline**. Warp commit `b0c7a767` has been imported, built, and
launched on macOS. The remaining M1 step is the explicitly approved `main` history migration; product
changes begin in M2.

See [the roadmap](docs/ROADMAP.md) for milestone scope and exit criteria.

## Documentation

- [Product](docs/PRODUCT.md): product contract, MVP scope, and non-goals
- [Architecture](docs/ARCHITECTURE.md): target boundaries, data flow, privacy, and permissions
- [Development](DEVELOPMENT.md): branches, specs, testing, and definition of done
- [Roadmap](docs/ROADMAP.md): ordered milestones with acceptance criteria
- [Baseline](docs/BASELINE.md): upstream commit, inventory, build evidence, and inherited failures
- [Agent instructions](AGENTS.md): repository rules for coding agents
- [Specs](docs/specs/README.md): when and how to write product and technical specs
- [Decisions](docs/decisions/README.md): architecture decision records

## Licensing

War retains Warp's upstream licensing: most client code is AGPL-3.0-only, while the `warpui_core` and
`warpui` crates are MIT licensed. War-specific code that forms part of the client will be
AGPL-3.0-only unless a file explicitly states otherwise.

See [NOTICE](NOTICE), [LICENSE-AGPL](LICENSE-AGPL), and [LICENSE-MIT](LICENSE-MIT) for attribution,
license, and trademark information.
