# AGENTS.md

Guide for AI coding agents (and humans) working on LazyArch.

## What this project is

LazyArch is an interactive post-install tool for Arch Linux, written in Rust. A beginner installs Arch, runs `lazy-arch`, and gets a usable system (drivers, audio, apps, dev tools) by picking options from a menu. Keep that user in mind: clear messages, safe defaults, nothing destructive without asking.

## Commands

```bash
cargo build                        # build
cargo clippy -- -D warnings        # must pass, CI fails on any warning
cargo test                         # must pass, CI runs it
cargo run -- --help                # check flags/menu text without touching the system
```

CI (`.github/workflows/ci.yml`) runs build, clippy and tests on every push and PR to `main`.

Features install packages and edit `/etc`, so **don't run them on your own machine to test**. Use an Arch VM. For a quick check, run `cargo run -- --help` and the `done()` checks, which are read-only.

## Layout

```
src/
├── main.rs      # --help/--version, Arch + non-root checks, dispatches CLI flags
├── menu.rs      # FEATURES table + CATEGORIES: menus, flags, --help, First Setup
├── system.rs    # run/sudo, pacman_install, is_installed, is_enabled, backup, write_root, detect_gpus
├── ui.rs        # colored output (info/success/warn/error), pick_packages
└── features/    # one file per feature
```

## Adding a feature

1. Create `src/features/<name>.rs` with:
   - `pub fn run()`: does the work.
   - `pub fn done() -> bool`: **read-only** check of the system (package installed, unit enabled, config line present). It drives the ✓ in the menu and the defaults in First Setup. Use `|| false` in the table for features without state, like lists or maintenance.
2. Add `pub mod <name>;` to `src/features/mod.rs` (alphabetical).
3. Add one `Feature { category, flag, name, run, done, first_setup }` line to `FEATURES` in `src/menu.rs`. The menu entry, CLI flag and `--help` come from it. The test in `menu.rs` fails if the category is missing from `CATEGORIES` or the flag is duplicated.
4. Update the Features list and the Usage block in `README.md` (paste the output of `cargo run -- --help`), and add a line to `CHANGELOG.md` under `[Unreleased]`.

Copy the shape of an existing feature: `ssd.rs` is the simple case, `languages.rs` is a package list, and `mirrors.rs` writes config.

## Rules

These come from decisions already made. Don't reverse them without the maintainer asking.

**Running commands**
- Always use `system::run`, `system::sudo` or `system::pacman_install`. Never use `Command::new(..).status().expect(..)`: it ignores non-zero exit codes. The helpers report failures and ask the user whether to continue.
- Use `system::succeeds` only for silent checks (inside `done()` and similar), never for actions.
- Never add `--noconfirm`. pacman should show what it will install and ask first.
- Use `pacman -Syu`, never `pacman -Sy`. A sync without an upgrade is an unsupported partial upgrade.
- The program refuses to run as root. Use `sudo` per command, and only where root is needed.

**Packages**
- Official repos only. The one exception is DaVinci Resolve, which uses the installed AUR helper (`aur::installed_helper()`).
- Check every package name before using it: `pacman -Si <name>` (or `pacman -Sg` for groups). Arch renames packages (`nvidia` became `nvidia-open`, and `cargo` ships inside `rust`). A wrong name makes pacman abort the whole transaction.
- Never download and run remote scripts (`curl | bash` and the like).

**Editing system files**
- Call `system::backup(path)` before the first edit of any config file. It creates `path.bak` once.
- Write root-owned files with `system::write_root(path, content)`, which pipes into `sudo tee`. Don't write a predictable temp file under `/tmp` and copy it.
- Edits must be idempotent: running a feature twice must not duplicate lines. Check before appending, like `pacman_config.rs` does with ILoveCandy and `shell.rs` with `append_missing`.
- Never delete user data (configs, keys, home directories). There is no revert or uninstall feature by design.

**Style**
- All user-facing text is in English. No i18n.
- Keep it small: lists of options are `const` tables `[(name, &[packages])]`, not new abstractions. Reuse the helpers in `system.rs` and `ui.rs` before writing new ones.
- No new crates unless the standard library and `dialoguer`/`colored` really can't do it.
- Match the surrounding code. Comments explain *why*, not *what*.
- Logic that can break quietly (parsing, escaping, generated files) gets one small `#[cfg(test)]` test. See `menu.rs`, `shell.rs` and `windows.rs`.

**Security**
- Anything that listens on a port binds to `127.0.0.1` (see `windows.rs`).
- Files that hold passwords are created with `0600` (data) or `0700` (scripts).
- Quote values in generated YAML and shell. Tests in `windows.rs` cover this.

## Commits and releases

- Small, focused commits with an imperative subject ("Add printer setup", "Fix multilib order").
- `main` is the only long-lived branch. PRs target `main`, and CI must be green before merging.
- To release:
  1. Bump `version` in `Cargo.toml`.
  2. Rename `[Unreleased]` in `CHANGELOG.md` to `[x.y.z] - YYYY-MM-DD`.
  3. Update "What's new" in `README.md`.
  4. Commit, then `git tag vX.Y.Z && git push && git push origin vX.Y.Z`.

  The `Release` workflow builds and publishes the binary that `Install.sh` downloads.
- Removing or renaming a CLI flag breaks users' scripts, so that change needs a major version.
