# LazyArch
![License](https://img.shields.io/github/license/MarceloAntonio/LazyArch)
![Last Commit](https://img.shields.io/github/last-commit/MarceloAntonio/LazyArch)
![Repo Size](https://img.shields.io/github/repo-size/MarceloAntonio/LazyArch)
![Arch Linux](https://img.shields.io/badge/arch-linux-blue)
![Rust](https://img.shields.io/badge/built%20with-rust-orange)

<p align="center">
  <img src="assets/logo.png" alt="LazyArch Logo" width="200"/>
</p>

---

## What is LazyArch?

LazyArch is a tool that automates boring or time-consuming installations and configurations on Arch Linux and Arch-based systems.

Instead of searching for commands and copying them repeatedly, LazyArch does everything for you. Just select an option, and it installs what you need along with all required dependencies.

The project follows the **KISS (Keep It Simple, Stupid)** philosophy, aiming for simplicity and a hassle-free experience.

> Previously written in Python, LazyArch has been fully rewritten in **Rust** — delivering a single native binary with no runtime dependencies.
> You can view the old repository by clicking [here](https://github.com/MarceloAntonio/LazyArch_old).

---

## Features

### Base System
- Pacman configuration (colors, parallel downloads, ILoveCandy)
- Mirror optimization via reflector (fastest up-to-date mirrors)
- Install an AUR helper (yay or paru)

### Hardware
- GPU driver auto-detection and installation (Intel, AMD, NVIDIA, hybrid laptops)
- Bluetooth setup with optional GUI (Blueman) and PipeWire support
- SSD TRIM activation with automatic weekly timer

### Desktop
- Nerd Fonts installer (JetBrains Mono, Fira Code, Hack, Iosevka, Cascadia Code)
- Shell switcher (Bash, Zsh, Fish, Nushell, Elvish, Tcsh)

### Development
- Programming language installer (Node.js, Go, Python, Java, Rust, PHP, C/C++, Ruby, Elixir, Zig, Lua, .NET)
- Docker + Docker Compose + Buildx setup
- Git configuration with SSH key generation (ed25519)

### Maintenance
- Remove orphaned packages
- Clean pacman cache
- Clean systemd journal logs
- Check failed services and boot errors

### Other
- Full **First Setup** wizard (runs all tasks in sequence)
- Gaming setup (Steam, Wine, Lutris, Gamemode, MangoHud)
- Colored terminal output
- Shows what pacman will install and asks before doing it
- Backs up `/etc/pacman.conf` before editing it
- `--version` and `--help` CLI flags
- Detects Arch and Arch-based distros automatically

---

## Requirements

- Arch Linux or an Arch-based distro (Manjaro, EndeavourOS, etc.)
- `pacman` package manager
- `sudo` privileges

---

## Installation

### Option 1: Quick install (recommended)

Downloads the pre-compiled binary directly from GitHub Releases — no Rust or compilation needed.

```bash
curl -fsSL https://raw.githubusercontent.com/MarceloAntonio/LazyArch/refs/heads/main/Install.sh | bash
```

### Option 2: Build from source

Requires Rust installed on your machine.

```bash
git clone https://github.com/MarceloAntonio/LazyArch
cd LazyArch
cargo build --release
./target/release/lazy-arch
```

---

## Usage

After installation, run from anywhere:

```bash
lazy-arch            # Start the interactive menu
```

You can also bypass the menu and run specific setup modules directly using CLI arguments:

```text
Usage: lazy-arch [OPTION]

Options:
  -v, --version       Show version
  -h, --help          Show this help
  --first-setup       Run the first setup wizard
  --pacman            Pacman Configuration
  --mirrors           Update Mirrors
  --aur               Install AUR Helper
  --gaming            GPU Drivers/Gaming Setup
  --bluetooth         Bluetooth Setup
  --ssd               SSD Trim Activation
  --fonts             Install Nerd Fonts
  --shell             Change Shell
  --languages         Language Installer
  --docker            Docker Setup
  --git               Git Setup
  --maintenance       System Maintenance
```

---

## Project Structure

```
src/
├── main.rs        # entry point, checks, CLI flags
├── ui.rs          # colored output helpers
├── menu.rs        # FEATURES table: menu, flags, --help, First Setup
├── system.rs      # run/sudo, pacman_install, backup, Arch detection
└── features/      # one file per feature, each exposes `pub fn run()`
    ├── aur.rs
    ├── bluetooth.rs
    ├── docker.rs
    ├── fonts.rs
    ├── gaming.rs
    ├── git.rs
    ├── languages.rs
    ├── maintenance.rs
    ├── mirrors.rs
    ├── pacman_config.rs
    ├── shell.rs
    └── ssd.rs
```

### Adding a feature

1. Create `src/features/<name>.rs` with a `pub fn run()`.
2. Add `pub mod <name>;` to `src/features/mod.rs`.
3. Add one line to `FEATURES` in `src/menu.rs`. The menu entry, CLI flag and `--help` text come from it.

Use `system::pacman_install` and `system::sudo`/`system::run` instead of `std::process::Command`, so failures are reported and the user can choose to continue.

---

## Uninstall

```bash
curl -fsSL https://raw.githubusercontent.com/MarceloAntonio/LazyArch/refs/heads/main/Uninstall.sh | bash
```
---

## Notes

- LazyArch is designed exclusively for **Arch Linux-based systems**
- The tool checks your distro automatically and warns if it's not Arch-based
