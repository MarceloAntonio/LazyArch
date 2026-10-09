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
- Pacman configuration (colors, parallel downloads, ILoveCandy), with a backup of `pacman.conf`
- Mirror optimization via reflector, optionally refreshed every week
- Install an AUR helper (yay or paru)
- Automatic time sync (NTP)

### Hardware
- GPU driver auto-detection and installation (Intel, AMD, NVIDIA, hybrid laptops) + gaming setup (Steam, Wine, Lutris, Gamemode, MangoHud)
- Audio with PipeWire
- Bluetooth setup with optional GUI (Blueman)
- SSD TRIM with automatic weekly timer
- Printer support (CUPS)
- Laptop power management (power-profiles-daemon or TLP)
- zram compressed swap

### Desktop
- Nerd Fonts installer
- Shell switcher (Bash, Zsh, Fish, Nushell, Elvish, Tcsh) with optional Starship prompt and Zsh plugins
- Flatpak + Flathub
- Common apps (browsers, media, office, chat, ...)
- DaVinci Resolve, Free or Studio (via the AUR helper)

### Development
- Programming language installer (Node.js, Go, Python, Java, Rust, PHP, C/C++, Ruby, Elixir, Zig, Lua, .NET)
- Docker + Docker Compose + Buildx
- Git configuration with SSH key generation (ed25519)
- Virtualization with QEMU/KVM and virt-manager
- Windows VM in Docker ([dockurr/windows](https://github.com/dockur/windows)), opened over RDP from your app menu

### Maintenance
- System update (pacman, AUR and Flatpak in one go)
- Remove orphaned packages, clean pacman cache (also weekly, automatically), clean journal logs
- Check failed services and boot errors

### Other
- **First Setup** wizard: pick the steps, already done ones start unchecked
- Detects what is already installed/configured (✓ in the menu) and asks before redoing it
- Shows what pacman will install and asks before doing it
- If a command fails, tells you and asks whether to continue
- Every feature can also run directly from the command line (see Usage)

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

Base System:
  --pacman            Pacman Configuration
  --mirrors           Update Mirrors
  --aur               Install AUR Helper
  --time-sync         Time Sync (NTP)

Hardware:
  --gaming            GPU Drivers/Gaming Setup
  --audio             Audio (PipeWire)
  --bluetooth         Bluetooth Setup
  --ssd               SSD Trim Activation
  --printer           Printer Setup
  --power             Laptop Power Management
  --zram              zram (Compressed Swap)

Desktop:
  --fonts             Install Nerd Fonts
  --shell             Change Shell
  --flatpak           Flatpak + Flathub
  --apps              Common Apps
  --davinci           DaVinci Resolve (AUR)

Development:
  --languages         Language Installer
  --docker            Docker Setup
  --git               Git Setup
  --virtualization    Virtualization (QEMU/KVM)
  --windows           Windows VM (Docker + RDP)

Maintenance:
  --update            System Update
  --maintenance       System Maintenance
```

---

## Project Structure

```
src/
├── main.rs        # entry point, checks, CLI flags
├── ui.rs          # colored output, package picker
├── menu.rs        # FEATURES table: menus, flags, --help, First Setup
├── system.rs      # run/sudo, pacman_install, backup, Arch detection
└── features/      # one file per feature: `pub fn run()` and `pub fn done()`
    ├── apps.rs
    ├── audio.rs
    ├── aur.rs
    ├── bluetooth.rs
    ├── davinci.rs
    ├── docker.rs
    ├── flatpak.rs
    ├── fonts.rs
    ├── gaming.rs
    ├── git.rs
    ├── languages.rs
    ├── maintenance.rs
    ├── mirrors.rs
    ├── pacman_config.rs
    ├── power.rs
    ├── printer.rs
    ├── shell.rs
    ├── ssd.rs
    ├── time_sync.rs
    ├── update.rs
    ├── virtualization.rs
    ├── windows.rs
    └── zram.rs
```

### Adding a feature

1. Create `src/features/<name>.rs` with a `pub fn run()` and a `pub fn done() -> bool` that checks the system.
2. Add `pub mod <name>;` to `src/features/mod.rs`.
3. Add one line to `FEATURES` in `src/menu.rs` with its category. The menu entry, CLI flag and `--help` text come from it.

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
