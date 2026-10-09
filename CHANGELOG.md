# Changelog

## [1.1.0] - 2026-10-09

### Added
- Menu grouped into category submenus
- Audio (PipeWire), Time Sync (NTP), Printer (CUPS), Flatpak + Flathub, Common Apps
- Laptop power management (power-profiles-daemon or TLP) and zram
- Starship prompt and Zsh plugins in Change Shell
- Virtualization (QEMU/KVM + virt-manager) and Windows VM (dockurr/windows + RDP)
- DaVinci Resolve through the AUR helper
- System Update (pacman, AUR, Flatpak), weekly mirror refresh and pacman cache cleanup timers
- Status checker: the menu marks finished features with ✓ and asks before running them again
- First Setup starts with already done steps unchecked
- Fonts and Languages pre-select what's already installed

### Changed
- Failed commands are now reported and LazyArch asks whether to continue (they used to be ignored)
- pacman shows what it will install and asks for confirmation (no more `--noconfirm`)
- Mirrors: reflector picks the fastest mirrors automatically instead of a fixed country list
- AUR: choose between yay and paru
- GPU: drivers are installed for every detected GPU (hybrid laptops), NVIDIA uses `nvidia-open`
- `/etc/pacman.conf` is backed up to `/etc/pacman.conf.bak` before the first edit
- Arch/root checks also apply to CLI flags

### Fixed
- Multilib is enabled before installing lib32 drivers
- `pacman -Sy` partial upgrades replaced with `pacman -Syu`
- ILoveCandy no longer duplicated on each run of the pacman configuration
- Rust install failed because of the nonexistent `cargo` package

### Removed
- Firewall, Desktop/WM installer, LazyVim and Proton-GE features
- Docker test environment and PKGBUILD

## [1.0.0]

### Added
- Pacman configuration (colors, parallel downloads, ILoveCandy)
- Mirror optimization via reflector
- AUR helper installation (yay)
- GPU driver auto-detection (Intel, AMD, NVIDIA)
- Gaming setup (Steam, Wine, Lutris, Gamemode, MangoHud, Proton-GE)
- Bluetooth setup with Blueman and PipeWire
- SSD TRIM activation
- Desktop/WM installer (i3, Hyprland, bspwm, GNOME, KDE Plasma)
- Nerd Fonts installer
- Shell switcher (Bash, Zsh, Fish)
- LazyVim with Catppuccin theme
- Programming language installer
- Docker + Compose + Buildx
- Git configuration + SSH key generation
- UFW firewall with profiles
- System maintenance tools
- First Setup wizard
- Colored terminal output
- `--version` and `--help` flags
- Arch/Arch-based distro detection
- Root execution prevention
