use std::fs;
use dialoguer::Select;
use crate::system::{is_enabled, pacman_install, sudo};
use crate::ui;

fn has_battery() -> bool {
    fs::read_dir("/sys/class/power_supply")
        .is_ok_and(|dir| dir.flatten().any(|e| e.file_name().to_string_lossy().starts_with("BAT")))
}

pub fn done() -> bool {
    is_enabled("power-profiles-daemon.service") || is_enabled("tlp.service")
}

pub fn run() {
    if !has_battery() {
        ui::warn("No battery found, power management is meant for laptops. Skipping.");
        return;
    }

    let options = [
        "power-profiles-daemon (simple, power modes in GNOME/KDE settings)",
        "TLP (advanced, more battery life, tweak in /etc/tlp.conf)",
    ];

    let choice = Select::new()
        .with_prompt("Select a power manager (they conflict, pick one)")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    if choice == 0 {
        pacman_install(&["power-profiles-daemon"]);
        sudo(&["systemctl", "enable", "--now", "power-profiles-daemon.service"]);
    } else {
        pacman_install(&["tlp"]);
        sudo(&["systemctl", "enable", "--now", "tlp.service"]);
        // TLP manages radios itself; systemd-rfkill would fight it (per TLP docs).
        sudo(&["systemctl", "mask", "systemd-rfkill.service", "systemd-rfkill.socket"]);
    }

    ui::success("Power management enabled!");
}
