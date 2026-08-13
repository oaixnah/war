# Product Spec: Local Terminal Startup

**Status:** Accepted
**Owner:** War maintainer
**Created:** 2026-08-12
**Accepted:** 2026-08-12

## Summary

War launches as a distinctly identified, account-free macOS application and opens a useful local
shell without requiring or contacting Warp services. Subsequent launches restore valid local terminal
windows and fall back to a fresh shell when restoration is unavailable or unsafe.

## Problem

The imported Warp OSS client starts with Warp identity, onboarding, login, hosted-service clients,
telemetry infrastructure, and a loopback web-integration listener. A user cannot treat that behavior as
a private local terminal, even when the login screen is skipped or networking is unavailable.

## Goals

- Launch directly into a local shell without an account, onboarding, conversion, or network access.
- Give War its own macOS application identity, storage, preferences, Keychain, logs, and URI scheme.
- Restore valid local terminal sessions without restoring excluded cloud product state.
- Make the absence of Warp, authentication, telemetry, and cloud-agent startup traffic auditable.

## Non-goals

- Add Agent mode or a model provider.
- Migrate or remain compatible with Warp or Warp OSS user data, settings, accounts, or credentials.
- Rename internal Rust crates, source modules, shell-integration protocol symbols, or repository-local
  `.warp` conventions.
- Complete the M5 deletion of every excluded module and dependency.
- Add a permanent runtime choice between local and cloud products.

## User Experience

On a first launch, War opens a window containing one terminal tab and starts the user's supported local
shell. No login, registration, onboarding, account conversion, or hosted-product prompt precedes the
terminal. The same flow works when all network interfaces are unavailable.

On a later launch, War restores valid local terminal windows and tabs according to the existing local
restore preference. If there is no restorable state, the state is corrupt, or a saved pane belongs to an
excluded cloud product, War opens a fresh local shell instead. A shell startup failure remains a local,
visible terminal error and never redirects the user to an account flow.

## Behavior Invariants

1. A fresh War installation launches directly into one usable local shell without showing login,
   onboarding, anonymous-account, conversion, billing, or hosted-agent UI.
2. Launch and normal terminal use behave the same with networking disabled; no network operation is a
   prerequisite for opening zsh or bash.
3. Normal startup does not initialize Warp Server, Firebase, GraphQL, Oz, billing, cloud-sync,
   cloud-agent, update, crash-upload, or product-telemetry services.
4. War makes no application-originated startup request and does not open the inherited loopback HTTP or
   local-control listener.
5. War uses the display name `War`, executable `war`, bundle identifier `tech.oaix.War`, URI scheme
   `war`, user configuration directory `~/.war`, and log file `~/Library/Logs/war.log`.
6. War does not read, migrate, link, modify, or delete Warp data under `.warp*`, `dev.warp.*`, the Warp
   application group, Warp UserDefaults suites, or Warp Keychain services.
7. A subsequent launch restores valid local terminal windows, tabs, and panes. Missing, corrupt, or
   excluded-product state falls back to one new local shell without blocking startup.
8. Retained local diagnostics do not upload data and do not record credentials, environment variables,
   terminal contents, prompts, or command output by default. Product telemetry is neither queued nor
   persisted.
9. zsh, bash, command blocks, tabs, history, search, copy, command re-entry, and alternate-screen
   applications remain usable after the startup change.
10. The visible application metadata, icon, menus, and startup surface do not imply affiliation with
    Warp. Upstream license texts and source attribution remain intact.

## Edge Cases

- If the preferred shell is missing, War uses the existing supported-shell fallback behavior.
- If a shell cannot start, the local terminal reports the failure and remains closable or retryable.
- If local persistence cannot be opened or decoded, War starts a new terminal and does not destroy the
  unreadable data automatically.
- Restored state containing remote, shared, Drive, notebook, agent, or cloud panes is not allowed to
  activate the corresponding service during startup.
- Shell startup files and commands may independently access the network; audits distinguish child-shell
  traffic from requests initiated by the War application.

## Privacy and Safety

No application data leaves the machine during startup or ordinary terminal use. War persists terminal
settings, history, and restorable local session state under its own application namespace. It does not
load inherited Warp credentials or telemetry queues. The terminal continues to execute user-entered
shell commands normally; this change adds no agent-requested side effects or approval behavior.

## Success Criteria

- Automated startup coverage creates and uses a terminal without registering excluded service models.
- Automated identity tests prove the canonical bundle, URI, path, and log names and reject inherited
  Warp OSS identity values.
- A clean macOS launch succeeds both online and offline with zsh and bash.
- A macOS process-level network audit observes zero application-originated startup traffic and no
  inherited loopback listener.
- Sentinel Warp OSS files, preferences, application-group data, and Keychain items remain untouched.
- The retained terminal regression suite and manual command-block, tab, and alternate-screen checks pass.

## Open Questions

None.
