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
- Mirror optimization via reflector (per-country)
- Install AUR helpers (yay)

### Hardware
- GPU driver auto-detection and installation (Intel, AMD, NVIDIA)
- Bluetooth setup with optional GUI (Blueman) and PipeWire support
- SSD TRIM activation with automatic weekly timer

### Desktop
- Desktop Environment / Window Manager installer (i3, Hyprland, bspwm, GNOME, KDE Plasma)
- Nerd Fonts installer (JetBrains Mono, Fira Code, Hack, Iosevka, Cascadia Code)
- Shell switcher (Bash, Zsh, Fish)

### Development
- LazyVim installation with optional Catppuccin theme
- Programming language installer (Node.js, Go, Python, Java, Rust, PHP, C/C++)
- Docker + Docker Compose + Buildx setup
- Git configuration with SSH key generation (ed25519)

### Security
- UFW firewall with profile presets (Personal, Developer, Custom)

### Maintenance
- Remove orphaned packages
- Clean pacman cache
- Clean systemd journal logs
- Check failed services and boot errors

### Other
- Full **First Setup** wizard (runs all tasks in sequence)
- Gaming setup (Steam, Wine, Lutris, Gamemode, MangoHud, Proton-GE)
- Colored terminal output
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
curl -sSL https://raw.githubusercontent.com/MarceloAntonio/LazyArch/refs/heads/main/Install.sh | bash
```

### Option 2: AUR (via PKGBUILD)

```bash
git clone https://github.com/MarceloAntonio/LazyArch
cd LazyArch
makepkg -si --noconfirm
```

### Option 3: Build from source

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
lazy-arch --help     # Show help
lazy-arch --version  # Show version
```

---

## Running with Docker (for testing)

If you want to test without touching your system:

```bash
# Build and enter the container
docker compose up -d
docker compose exec rust-dev bash

# Inside the container
cargo run
```
> Before running Lazy Arch, run the following command to change the password for the dev user and set a password of your choice: <br>`sudo passwd dev`

---

## Project Structure

```
src/
├── main.rs
├── ui.rs
├── menu/
│   ├── mod.rs
│   └── menu.rs
├── features/
│   ├── mod.rs
│   ├── aur.rs
│   ├── bluetooth.rs
│   ├── desktop.rs
│   ├── docker.rs
│   ├── firewall.rs
│   ├── fonts.rs
│   ├── gaming.rs
│   ├── git.rs
│   ├── languages.rs
│   ├── lazyvim.rs
│   ├── maintenance.rs
│   ├── mirrors.rs
│   ├── pacman.rs
│   ├── shell.rs
│   └── ssd.rs
└── system/
    ├── mod.rs
    ├── is_arch.rs
    ├── is_systemd_running.rs
    └── pacman.rs
```

---

## Uninstall

```bash
curl -sSL https://raw.githubusercontent.com/MarceloAntonio/LazyArch/refs/heads/main/Uninstall.sh | bash
```
---

## Notes

- LazyArch is designed exclusively for **Arch Linux-based systems**
- The tool checks your distro automatically and warns if it's not Arch-based
