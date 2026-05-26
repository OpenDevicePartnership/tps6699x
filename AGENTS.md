# AGENTS.md — tps6699x

> Operational guide for AI coding agents working on
> [`openDevicePartnership/tps6699x`](https://github.com/openDevicePartnership/tps6699x).
> Humans should also find this useful; it is the single source of truth for
> repository conventions and supersedes `.github/copilot-instructions.md`.

This file follows the [agents.md](https://agents.md/) convention. Read it
before you make any changes.

---

## 1. Project overview

`tps6699x` is a `no_std`, embedded Rust driver for the Texas Instruments
TPS6699x family of USB Type-C / USB-PD controllers (TPS66993 — 1 port,
TPS66994 — 2 ports). It is part of the
[OpenDevicePartnership](https://github.com/OpenDevicePartnership) ecosystem and
talks to the controller over I²C, on top of the `embedded-hal` /
`embedded-hal-async` traits and the shared
[`embedded-usb-pd`](https://github.com/OpenDevicePartnership/embedded-usb-pd)
crate.

Key facts:

- Edition `2021`, MSRV **1.88** (declared in `Cargo.toml` and enforced by CI).
- Default target for examples and `rust-toolchain.toml`: `thumbv8m.main-none-eabihf`.
- License: MIT.
- Cargo features:
  - `defmt` — `defmt` logging via the `fmt` macros.
  - `log` — `log` crate logging via the `fmt` macros (mutually compatible with `defmt`; one or the other typically).
  - `embassy` — pulls in `embassy-sync`, `embassy-time`, and `heapless`; enables the high-level async API under `src/asynchronous/embassy/`.
- Register map is **generated** by [`device-driver`](https://crates.io/crates/device-driver) from `device.yaml` into `src/registers/generated.rs`. **Do not hand-edit `generated.rs`.**

---

## 2. Repository layout

```
.
├── AGENTS.md                ← this file
├── Cargo.toml               ← crate manifest, features, lints
├── CODEOWNERS, CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md
├── LICENSE                  ← MIT
├── README.md                ← short, points at device-driver regeneration
├── deny.toml                ← cargo-deny config
├── device.yaml              ← register manifest (source of truth for registers)
├── rust-toolchain.toml      ← stable + thumbv8m.main-none-eabihf target, clippy/rustfmt/llvm-tools
├── rustfmt.toml             ← max_width=120, StdExternalCrate import grouping
├── supply-chain/            ← cargo-vet audits/imports
├── .github/
│   ├── copilot-instructions.md   ← short PR-review pointer (see §11)
│   └── workflows/
│       ├── check.yml             ← fmt/clippy/doc/check (hack)/deny/msrv/check-examples/test/test-doc
│       ├── nostd.yml             ← cargo check --target thumbv8m.main-none-eabihf --no-default-features
│       ├── device-driver.yml     ← regenerates registers and diffs against committed file
│       ├── cargo-vet.yml         ← cargo vet --locked
│       └── cargo-vet-pr-comment.yml
├── examples/
│   └── rt685s-evk/          ← NXP RT685 EVK firmware examples (separate Cargo.toml)
│       └── src/bin/{plug_status.rs, fw_update.rs}
└── src/
    ├── lib.rs               ← #![no_std], top-level types (Mode, DeviceError, constants)
    ├── fmt.rs               ← defmt/log shim macros (trace!/debug!/info!/warn!/error!)
    ├── fw_update.rs         ← firmware-update data plane (TFUx parsing)
    ├── stream.rs            ← streaming helpers
    ├── command/             ← typed command argument structs (gcdm, muxr, trig, vdms)
    ├── registers/           ← device-driver-generated + hand-written field types
    │   ├── generated.rs     ← GENERATED; regenerate via device-driver-cli
    │   └── *.rs             ← typed wrappers around generated register field-sets
    └── asynchronous/
        ├── mod.rs           ← async surface (`internal`, `interrupt`, `fw_update`)
        ├── internal/        ← low-level async controller + commands
        ├── interrupt.rs     ← InterruptController trait
        ├── fw_update.rs     ← async TFU sequencing
        └── embassy/         ← high-level Embassy API (requires `embassy` feature)
            ├── mod.rs       ← `Controller`/`Tps6699x` user-facing types
            ├── interrupt.rs, task.rs, fw_update.rs, rx_caps.rs, ucsi.rs
```

---

## 3. Toolchain & environment

- Use the toolchain pinned by `rust-toolchain.toml` (currently the host's
  default `stable`, with the `thumbv8m.main-none-eabihf` target and `rust-src`,
  `rustfmt`, `clippy`, `llvm-tools-preview` components). Running any `cargo`
  command in the repo root will install/select these automatically.
- The `fmt` CI job uses **nightly** rustfmt. If `cargo fmt --check` passes on
  nightly but you're on stable, install nightly with `rustup toolchain install
  nightly --component rustfmt` and run `cargo +nightly fmt --check`.
- The `doc` CI job uses nightly with `RUSTDOCFLAGS=--cfg docsrs`. Reproduce
  locally with `RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --all-features`.
- `cargo-hack`, `cargo-deny`, `cargo-vet`, `device-driver-cli`, and `grcov`
  are used by CI; install on demand with `cargo install <tool>` if you need to
  reproduce a job locally.

---

## 4. Build, test, lint, doc

The list below mirrors `.github/workflows/check.yml`,
`.github/workflows/nostd.yml`, and `.github/workflows/device-driver.yml`
exactly. Run these before pushing.

### Format

```sh
cargo fmt --check                 # CI uses nightly; run cargo +nightly fmt --check if available
```

### Clippy

```sh
cargo clippy                      # stable + beta in CI; lint config lives in Cargo.toml [lints.clippy]
```

The crate denies (in `Cargo.toml`): `correctness`, `expect_used`,
`indexing_slicing`, `panic`, `panic_in_result_fn`, `perf`, `suspicious`,
`style`, `todo`, `unimplemented`, `unreachable`, `unwrap_used`. Do **not**
introduce `unwrap()`, `expect()`, `panic!()`, raw indexing (`a[i]`), `todo!()`,
`unimplemented!()`, or `unreachable!()` in `src/`. Use `?` with `PdError` /
`DeviceError`, `get(..).ok_or(...)?`, etc. In `#[cfg(test)]` code these are
permitted.

### Check (every feature combination)

```sh
cargo hack --feature-powerset check
```

### Tests

```sh
cargo hack --feature-powerset --exclude-features defmt test --all-targets
cargo test --doc
```

`defmt` is excluded from the powerset test job because `defmt` and `log`
collide at link time when both are active in a host build.

### `no_std` target check

```sh
rustup target add thumbv8m.main-none-eabihf
cargo check --target thumbv8m.main-none-eabihf --no-default-features
```

### Examples

```sh
cd examples/rt685s-evk
cargo check --target thumbv8m.main-none-eabihf
```

### Doc

```sh
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --no-deps --all-features
```

### MSRV

```sh
rustup toolchain install 1.88
cargo +1.88 check
```

### Supply chain (cargo-deny / cargo-vet)

```sh
cargo deny --manifest-path ./Cargo.toml check --all-features
cargo vet --locked
```

### Regenerating the register map (only when `device.yaml` changes)

```sh
cargo install device-driver-cli --version 1.0.9
device-driver-cli --manifest device.yaml --device-name Registers \
    -o src/registers/generated.rs
```

Commit `device.yaml` and the regenerated `src/registers/generated.rs` in the
same change. The `device-driver-pregen-check` workflow re-runs this generator
and diffs against the committed file; any drift fails CI.

---

## 5. Coding conventions

### Style

- `rustfmt.toml`: `max_width = 120`, `imports_granularity = "Module"`,
  `group_imports = "StdExternalCrate"`. Always run `cargo fmt` before
  committing.
- Always prefer fallible APIs over panicking ones. See the deny-list above.
- Use the crate-local logging macros from `src/fmt.rs` (`trace!`, `debug!`,
  `info!`, `warn!`, `error!`) so that both `defmt` and `log` users are
  supported transparently.
- Public types and functions should carry `///` doc comments. The `doc` CI job
  runs with `--cfg docsrs`; if you add platform-conditional items, gate them
  with `#[cfg_attr(docsrs, doc(cfg(...)))]`.
- Mirror the existing `#[cfg_attr(feature = "defmt", derive(defmt::Format))]`
  pattern on new public enums/structs so they can be logged with `defmt`.

### `no_std`

- `src/lib.rs` is `#![no_std]`. Do not pull in `std` from `src/`. Use `core`
  and (where the `embassy` / `heapless` features allow) the fixed-capacity
  collections from `heapless`.
- `extern crate std;` is only allowed under `#[cfg(test)]`, mirroring
  `src/lib.rs::test`.
- Anything new under `src/asynchronous/embassy/` must be gated by the
  `embassy` feature (`#[cfg(feature = "embassy")]` at the module boundary —
  see how `src/asynchronous/mod.rs` already gates `pub mod embassy;`).

### Async safety

The repository specifically calls out two areas to review carefully (from
`.github/copilot-instructions.md`, reproduced here so agents see it):

- Code that uses async selection APIs such as `select`, `selectN`,
  `select_array`, `select_slice`, or is marked with a drop-safety comment.
  Those helpers drop the futures that don't finish; verify that values
  (responses, partial state) are not lost when the loser future is dropped.
- Code that could panic or is marked with a panic-safety comment.

When you write or modify drop-sensitive async code, leave a comment that
explains why dropping is safe (or document the invariant that prevents loss).

### Error handling

- The crate exposes `DeviceError<BE, T>` in `src/lib.rs` wrapping
  `embedded_usb_pd::Error<BE>` and a custom `Other(T)`. New driver-level
  errors should slot into this or `embedded_usb_pd::PdError` rather than
  introducing a parallel error type.
- I/O errors from the bus implement `From` into `embedded_usb_pd::Error<BE>`;
  use `?` to propagate.

### Registers

- The single source of truth for register layout is `device.yaml`. Treat
  `src/registers/generated.rs` as build output: do not edit by hand, and
  always regenerate via `device-driver-cli` (§4) and commit both files
  together.
- Per-register convenience types (newtypes, conversions, semantic enums) live
  in sibling files under `src/registers/` (e.g. `port_config.rs`,
  `boot_flags.rs`). Add new wrappers there, not in `generated.rs`.

### Commands

- Typed command argument structs and helpers live in `src/command/`. New
  commands should add a module (or extend an existing one) and reuse the
  `bincode` `encode_into_slice` pattern already used for `TfuiArgs` /
  `TfudArgs` (`bincode::config::standard().with_fixed_int_encoding()`).

---

## 6. Testing

- Unit tests live next to the code in `#[cfg(test)] mod tests`. Shared test
  helpers are in `crate::test` (re-exported from `src/lib.rs` behind
  `#[cfg(test)]`) — reuse `create_register_read` / `create_register_write` and
  the `Delay` helper rather than rolling your own.
- The bus is mocked with `embedded-hal-mock` (`eh1::i2c::Transaction`). New
  tests for I²C interactions should follow the existing pattern: build a
  transaction list, instantiate the controller around it, drive the operation,
  then call `i2c.done()`.
- Async tests use `tokio` (with `rt`, `macros`, `time`) plus the `embassy`
  feature where Embassy primitives are required. `critical-section` with the
  `std` feature is enabled in `dev-dependencies` to make `embassy-sync`
  primitives usable on the host.
- Coverage is collected in CI via `grcov` with `-C instrument-coverage`; you
  do not need to run this locally unless you are debugging the coverage job.

When adding a behavior, write a test that fails before your change and passes
after — particularly for protocol/command sequences where regressions are
otherwise invisible.

---

## 7. Examples

`examples/rt685s-evk/` is a separate Cargo project that targets the NXP
RT685 EVK and depends on `embassy-imxrt`. CI only runs `cargo check
--target thumbv8m.main-none-eabihf` against it; do the same locally if you
touch shared driver APIs. The example binaries (`plug_status`, `fw_update`)
are the closest thing to integration tests for the high-level Embassy API.

---

## 8. Commit conventions

From `.github/copilot-instructions.md` and `CONTRIBUTING.md`:

- **Subject**: capitalized, ≤ 50 characters, imperative mood ("Fix bug", not
  "Fixed bug"). No trailing period.
- **Blank line** between subject and body.
- **Body**: wrap at 72 characters; explain *what* and *why*, not *how*.
- Each commit must build cleanly without warnings; squash typo/format fixups
  before pushing.
- **Do not** force-push to shared branches.

### AI attribution (mandatory for AI-generated or AI-assisted commits)

Every commit produced wholly or in part by an AI agent **must** end with an
`Assisted-by` trailer:

```
Assisted-by: AGENT_NAME:MODEL_VERSION [TOOL1] [TOOL2]
```

- `AGENT_NAME` — e.g. `GitHub Copilot`.
- `MODEL_VERSION` — the *actual* model identifier you are running as. Verify
  it before composing the trailer; never hard-code a value carried over from a
  previous session.
- `[TOOLn]` — optional specialized analysis tools (e.g. `coccinelle`,
  `sparse`, `smatch`, `clang-tidy`). Basic dev tools (git, cargo, editors) do
  **not** belong here.

AI agents **must not** add `Signed-off-by` trailers — only humans can certify
the Developer Certificate of Origin.

### CONTRIBUTING.md highlights

- Open a **draft PR** first; confirm `.github/` workflows pass on the draft
  before requesting review.
- Maintain a clean linear history — squashing is disabled on `main`. Each
  commit must build without warnings; squash trivial fixups locally.
- When reporting regressions, use `git bisect` to pinpoint the first bad
  commit.

---

## 9. PR review focus

CI itself flags compile errors, compiler warnings, and clippy warnings; you do
not need to call those out in review comments. Reviewers (human or AI) should
concentrate on:

- **Async drop safety** around `select`/`selectN`/`select_array`/`select_slice`
  and any code annotated with a drop-safety comment. Verify the dropped
  future cannot lose data the program relied on.
- **Panic-freedom**: any new `unwrap`/`expect`/`panic!`/indexing/`todo!`/
  `unimplemented!`/`unreachable!` outside `#[cfg(test)]` is a bug. Suggest
  fallible alternatives (`?`, `ok_or`, `get`, `try_into`, …).
- **Register/command correctness**: edits in `src/registers/` (other than
  `generated.rs`) and `src/command/` should match the TI register/command
  semantics. If `device.yaml` changed, was `generated.rs` regenerated and
  committed (see §4)?
- **Feature flag hygiene**: every feature combination must compile (`cargo
  hack --feature-powerset check`). Items that depend on `embassy`, `defmt`,
  `log`, or `heapless` must be properly `#[cfg(...)]`-gated.
- **MSRV (1.88)**: do not use APIs stabilized after 1.88.
- **`no_std` purity** in `src/` (no `std::`, no host-only crates outside
  `dev-dependencies`).
- **Supply chain**: new dependencies need a `cargo vet` entry; license/ban
  changes go through `deny.toml`.

---

## 10. Workflow for agents

1. **Sync** from `upstream/main` before starting work. Create a topic branch
   (`git checkout -B <topic> upstream/main`).
2. **Plan** the change in the smallest reviewable slice. Prefer many small,
   focused commits over one large one — `main` keeps history linear.
3. **Implement** the change. If you touch `device.yaml`, regenerate
   `src/registers/generated.rs` in the same commit (§4).
4. **Validate locally** with at least:
   - `cargo fmt --check` (nightly if available)
   - `cargo clippy`
   - `cargo hack --feature-powerset check`
   - `cargo hack --feature-powerset --exclude-features defmt test --all-targets`
   - `cargo test --doc`
   - `cargo check --target thumbv8m.main-none-eabihf --no-default-features`
   - `(cd examples/rt685s-evk && cargo check --target thumbv8m.main-none-eabihf)` if the change touches the public driver API.
   Document which commands you ran in the PR description.
5. **Commit** with the conventions in §8, including the `Assisted-by` trailer
   when AI-assisted. Set the author to the human you are acting on behalf of;
   never commit as the AI itself.
6. **Push** to your fork on a topic branch. Do not force-push to `main`;
   prefer additional commits over rewrites once the branch is shared.
7. **Open a draft PR** per `CONTRIBUTING.md`; let CI go green before
   requesting review. Cross-link any related upstream issues.
8. **Stop** at the point the user asked for — for example, "push to fork" does
   not include opening a PR unless explicitly requested.

---

## 11. Relationship to `.github/copilot-instructions.md`

`.github/copilot-instructions.md` is the historical, short PR-review pointer
for GitHub Copilot. This file is a strict superset of it. If the two ever
disagree, `AGENTS.md` wins; please update both in the same commit and keep the
pointer at the top of `copilot-instructions.md` intact.

---

## 12. Things to flag, not block on

- An existing `AGENTS.md` that conflicts with this file — reconcile rather
  than silently overwrite.
- Drift between `device.yaml` and `src/registers/generated.rs` (the
  `device-driver-pregen-check` workflow will already fail, but flag it in
  review).
- CRLF line endings on files that the rest of the tree stores as LF — this
  repository targets LF (`core.autocrlf=false`). Re-save offending files.
- Broken CI on `main` not introduced by the current change — mention it but
  do not block the change in flight.
- Missing or stale `.github/copilot-instructions.md` — re-add the pointer
  block described in §11 rather than removing the file.
