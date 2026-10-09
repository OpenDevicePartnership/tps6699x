---
name: code-review
description: Use when reviewing a pull request or proposed change in the tps6699x repository — Rust driver code, device.ddsl, examples, tests, docs, CI workflows or dependency bumps — including hardware-documentation claims, register-codegen and feature-flag invariants, async cancellation safety, firmware updates, or supply-chain risks.
---

# Reviewing changes to the tps6699x driver

## What this crate is

`tps6699x` is a `#![no_std]` driver for Texas Instruments TPS6699x series
USB Type-C and USB Power Delivery controllers, using `embedded-hal` and
`embedded-hal-async` 1.0.

- `src/asynchronous/` contains async driver, interrupt and firmware-update
  code; Embassy integration is gated by the `embassy` feature.
- `src/registers/generated.rs` is **generated** from `device.ddsl` by
  `device-driver`. It is committed but never hand-edited.
- `src/registers/` also contains hand-written register wrappers and
  conversions; `src/command/` contains command definitions.
- `src/fw_update.rs` and `src/stream.rs` support firmware-image handling.
- `src/odp/` integrates with other ODP crates under
  `odp-embedded-services`, `odp-fw-update-interface` and `odp-tcpm-service`.
- `Cargo.toml` defines the actual lint policy. Generated-code allowances
  are scoped to `mod generated` in `src/registers/mod.rs`; do not mistake
  existing generated-code patterns for newly introduced defects.

**The driver's job is to be faithful to the chip, not to be clever.** A
change that is elegant Rust and wrong about the hardware is a defect.

## Orient before commenting

Read, in this order, stopping when you have enough to judge the diff:

1. Applicable `AGENTS.md` files, if present, and
   `.github/copilot-instructions.md`. The latter directs reviewers not to
   comment on compile errors, compiler warnings or clippy warnings, and
   specifically calls out cancellation and panic safety.
2. `README.md` for register regeneration and ODP integration;
   `CONTRIBUTING.md` for contribution and commit conventions.
3. Any spec matching the change, if one exists under `docs/`. Do not assume
   that a spec directory or a particular design document exists.
4. `device.ddsl` and the relevant hand-written register wrappers for layout,
   encoding, decoding or validation changes.
5. The affected driver, command, interrupt, firmware-update or ODP module
   and its tests. Follow a changed operation through its callers and error
   paths, not just the edited lines.
6. `Cargo.toml` and the relevant `.github/workflows/` files for feature,
   dependency, codegen or CI changes.

A comment that contradicts an established contract is a false positive.
Check before filing it; if a contract conflicts with vendor documentation,
report that conflict with evidence rather than assuming either is infallible.

## Vendor documents are the source of truth

Every `.txt` file under `docs/vendor/` is a pre-extracted vendor document and
is the citable source for hardware claims. List the directory before reviewing
— application notes, errata, reference manuals and other documents may have
been added since this skill was written.

The available extraction includes:

| File | Document | Revision |
|---|---|---|
| `docs/vendor/slvsfm6a.txt` | TPS65994AD Dual Port USB Type-C and USB PD Controller datasheet | SLVSFM6A, August 2020, revised July 2021 |

**Check device applicability.** The crate targets the TPS6699x series, while
this extract identifies TPS65994AD. Do not silently treat that document as
proof of every register, command or behavior on every supported variant.
State the applicability gap when the available documentation does not
establish a claim for the target device.

The extracts reproduce TI's copyrighted documentation; the repository's MIT
license does not relicense them. They are review references, not replacements
for TI's official publications. Use the existing extracts; do not re-download
or regenerate them as routine review setup. If provenance documentation exists,
consult it when an extract appears stale or incomplete. Otherwise report the
specific gap, without inventing a provenance file or regeneration requirement.

### Citing a hardware fact

**Grep the extract. Never cite from memory.** Every hardware claim in a review
comment carries a `docs/vendor/<file>.txt:<line>` anchor and a verbatim quote.
For example, the unique-address interface description states:

> "The TPS65994AD Host interface utilizes a different unique address to identify"
> "each of the two USB Type-C ports controlled by the TPS65994AD."
> (`docs/vendor/slvsfm6a.txt:2645-2646`)

Quote verbatim, and give a range when the sentence wraps across lines. For
protocol diagrams and tables, cite the relevant labels and figure or table
number instead of inventing prose that is not in the extract.

If a fact is not in any available vendor extract, **do not assert it as a
vendor requirement.** Say the behavior is undocumented in the available
sources and ask the author what it is based on. Source-code evidence can
establish the implementation's behavior, but not prove the chip's behavior.

### Hardware and protocol checks

| Change touches | Verify |
|---|---|
| I²C addressing and port selection | Correct device variant, port count, address selection and bounds; the unique-address discussion is at `docs/vendor/slvsfm6a.txt:2641-2647`. |
| Register transfers | Register number, byte count, payload size and read/write transaction sequence; Figures 8-25 and 8-26 are at `docs/vendor/slvsfm6a.txt:2655-2676`. |
| Register or command encoding | Address, access mode, field width, byte order, reserved bits, reset values and four-byte command representation; find a supporting document before asserting device requirements. |
| Interrupt handling | Event/mask/clear semantics, event ownership, races between sampling and clearing, and whether a new read or write consumes state. |
| Command execution | Command submission, completion, timeout, return status and recovery after partial bus failure or cancellation. |
| Firmware updates | Image bounds, lengths and offsets, block numbering, update-mode transitions, timeouts and recovery after a partial transfer. |
| PD/Type-C and ODP conversions | Units, enum validity, per-port state, public trait contracts and behavior for unsupported values. |

Do not import assumptions about destructive reads, byte order or reset values
from another device. Verify each against documentation applicable to the change.

## Repository invariants

| Invariant | What a violating diff looks like |
|---|---|
| Register bindings are generated | Hand edits to `src/registers/generated.rs` instead of a `device.ddsl` change and regeneration. Generator-version changes can legitimately alter output without changing the DDSL. |
| Codegen settings stay aligned | `README.md` and `.github/workflows/device-driver.yml` disagree on the CLI version or omit `--rust-defmt-feature=defmt`; the generated bytes no longer match CI regeneration. |
| Feature gates follow `Cargo.toml` | Optional Embassy or ODP dependencies referenced outside their gates; changes that break feature additivity or integration feature implications. |
| Public API includes re-exported register types | A removed or renamed public item or conversion is dismissed as internal despite being reachable through `pub use generated::*` or another public module. |
| Async selection may drop unfinished futures | New `select`-family operations lose an event, buffer, command or state transition when the losing future is dropped. Follow drop-safety comments and the actual cancellation boundary. |
| Input and state errors must not become panics | Unchecked indexing, casts, image offsets or unexpected enum values become reachable from bus responses, firmware images or caller inputs. Distinguish production code from test assertions. |
| Dependency trust is reviewed | New or changed dependencies lack appropriate `cargo-vet` coverage, or an audit/trust change weakens the required criteria. Inspect the actual coverage rather than requiring a new entry for already-covered versions. |
| Commit conventions are local | Apply `.github/copilot-instructions.md` and `CONTRIBUTING.md`; AI-assisted commits require `Assisted-by:` and agents must not add `Signed-off-by:`. |

Use the current CI commands as the check contract, not a copied feature matrix.
In particular, `.github/workflows/check.yml` uses
`--mutually-exclusive-features=log,defmt` for feature-powerset clippy and
`--exclude-features defmt` for feature-powerset tests. Record which checks
actually ran; do not imply unexecuted checks passed. Do not duplicate
compiler or clippy diagnostics as review findings.

## Adversarial pass

Run this on every diff, including ones that look like pure documentation.
Report what the code does and what it would enable; do not accuse anyone of
intent.

Investigate when a diff:

- Adds or redirects a `git`, `path` or `[patch]` dependency, changes a trusted
  source or revision, or introduces a name resembling a popular crate.
  This repository already uses ODP Git dependencies; their mere presence
  is not a new finding. Evaluate what the diff changes.
- Adds a `build.rs` or other build-time executable. Ask why it is needed and
  inspect its file, network, process and environment access.
- Adds `unsafe` to hand-written code or widens a lint allowance beyond the
  generated module. Ask what warning is being silenced and why a fix is
  not possible; existing generated `unsafe` is not itself a defect.
- Adds file, network, process or environment access — `std::process`,
  `std::net`, `std::fs`, `env!`, `option_env!`, `include_str!`,
  `include_bytes!` — unrelated to the driver's purpose or a documented
  example/test use. This is a `no_std` hardware driver; justify external
  access rather than treating legitimate firmware-image inputs as attacks.
- Changes `.github/workflows/`. Check for `pull_request_target` combined
  with execution of PR code, new secret access or secrets logged, mutable
  action references or forks, remote code execution such as `curl … | sh`,
  and expanded permissions, especially `id-token: write` or `contents: write`.
- Adds or redirects a publishing path. Review credentials, destinations,
  triggers and artifact provenance; do not assume release tooling exists.
- Weakens `supply-chain/` — removes audits, relaxes required criteria or
  adds unexplained publisher trust.
- Contains characters that do not render as they read: zero-width joiners,
  bidirectional overrides, homoglyphs or unexplained base64/hex blobs.
  Quote escaped bytes in the comment.
- Adds logic keyed to an address, date, environment variable or build
  profile so that behavior differs between CI and a user's machine.

None of these are automatically malicious. A finding needs a concrete risk
and evidence, not just a keyword match or speculation about intent.

## Writing the review

Each comment is four things, in this order:

1. **The claim** — one sentence stating what is wrong.
2. **The evidence** — a vendor-extract line anchor with a verbatim quote
   for hardware claims; an applicable instruction, contract or `path:line`
   for repository claims.
3. **The consequence** — what a user observes, what state is lost, or
   which repository contract is violated.
4. **The fix** — concrete, and a suggested change when it is a few lines.

Severity:

| Level | Use for |
|---|---|
| **High** | A hardware-visible contradiction of applicable vendor documentation; wrong register address, encoding or clearing rule; lost state during cancellation; unsafe firmware-update behavior; a concrete supply-chain risk; a silent public API break. |
| **Medium** | A latent defect, violated repository contract, or missing regression coverage for behavior introduced by the diff. |
| **Low** | Documentation or contract gaps: correct behavior whose rules are unstated, a stale comment or a missing error note. |

Prefer few, well-anchored comments over many thin ones. If the diff is clean,
say so and name what you verified, which vendor documents you checked, and
any applicability or verification limits.

## Known non-findings and common mistakes

- **Generated register code is repetitive or carries scoped allowances.**
  Review the DDSL and generator configuration, not the generator's style.
- **`REG_DATA1` is hand-written outside the generated mapping.**
  This is the current implementation, not proof of a generator limit.
  `device-driver` 2.1.1 supports a 64-byte (512-bit) fieldset, including
  `field Bytes[64 stride 8] 7:0`. Individual integer fields are limited to
  64 bits, so a single `field Data 511:0` is rejected. The size-limit
  comments in `src/registers/mod.rs` and
  `src/asynchronous/internal/command.rs` are outdated as register-size claims.
  Do not demand migration merely for style; direct slice access also supports
  variable-length command payloads, which a fixed-size fieldset does not
  automatically preserve.
- **The crate already has Git dependencies on ODP crates.** A new source
  or trust change needs review; unchanged dependencies are not diff findings.
- **There is no blocking counterpart to an async operation.** Do not invent
  a sync/async parity requirement absent from this repository's contracts.
- **The PR does not use Conventional Commits or release-plz.** Neither is
  required by the current local contribution instructions.

## Red flags — stop and check

- "The datasheet says…" with no line anchor → grep `docs/vendor/` first.
- A claim about a TPS6699x device based solely on the TPS65994AD extract →
  establish applicability or state the gap.
- "This should use a newtype / a different name" with no defect → tie it
  to a local contract or drop it.
- "This is a breaking change" for a private item → check its public reachability.
- "Only the datasheet matters" → every available vendor extract is in scope;
  application notes and errata may qualify datasheet requirements.
- A new async selection or drop-safety comment → trace the losing future's
  state and cleanup before accepting the change.
- A documentation-only diff → still run the adversarial pass. Instruction
  files and this skill guide agents with tool access; changing them changes
  agent behavior.
