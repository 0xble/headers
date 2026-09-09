# Maintenance

## Background

Maintained source fork: `0xble/headers` of `transmissions11/headers`; both use
`master`. This temporary clone was inspected from owned `origin/master`
`3a504c6a77ad20c65e20825739ff65b04993ee4e`; accepted upstream baseline:
`dba4754d507eda378a713195c2e21712b2dd1845`. Publish only to `origin`, never
upstream. Source publication and any installed CLI are separate stages.

## Preserve

- Header output keeps the local customization and correctly centers odd-length
  headers without padding drift.

## Active patches

### HEADERS-001: `customize`

- **Provenance:** `052b680b0943a9febde7a5ef3b7fb0cf5c42da7a`.
- **Surfaces:** `src/main.rs`.
- **Upstream issue / PR:** None after checked 2026-09-09 / None after checked 2026-09-09.
- **Regression:** Blocked: no assertion defines the intended local customization; add one before reconciliation or publication.
- **Rollback:** Revert `052b680b0943a9febde7a5ef3b7fb0cf5c42da7a` and run the complete gate.
- **Retire when:** a released upstream behavior intentionally replaces this customization and its assertion.

### HEADERS-002: `fix padding for headers with odd length`

- **Provenance:** `3a504c6a77ad20c65e20825739ff65b04993ee4e`.
- **Surfaces:** `src/main.rs`.
- **Upstream issue / PR:** None after checked 2026-09-09 / None after checked 2026-09-09.
- **Regression:** Blocked: no odd-length output assertion exists; add one before reconciliation or publication.
- **Rollback:** Revert `3a504c6a77ad20c65e20825739ff65b04993ee4e` and run the complete gate.
- **Retire when:** an upstream release carries equivalent padding behavior with an assertion.

## Update and verify

Every run fetches owned `master` and latest upstream `master`, reconciles only
these patches, runs `cargo test` and `cargo build`, and publishes only after both
blocked regressions are resolved. Immediately fetch upstream again;
`git rev-list --left-right --count upstream/master...master` must show zero
upstream-only commits. After authorized publication require local/`origin/master`
SHA parity, otherwise report `Blocked` with the exact refs and failed proof. Do
not install or validate a runtime without separate authorization and exact
runtime-SHA proof.
