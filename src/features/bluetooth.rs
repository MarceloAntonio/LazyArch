use std::process::Command;
use dialoguer::Confirm;
use crate::system::pacman::pacman_install;
use crate::system::is_systemd_running::is_systemd_running;
use crate::ui;

pub fn bluetooth_setup() {
    let mut packages = vec!["bluez", "bluez-utils"];

    let gui = Confirm::new()
        .with_prompt("Install Blueman (GUI manager)?")
        .default(true)
        .interact()
        .unwrap();

    if gui {
        packages.push("blueman");
    }

    let pipewire = Confirm::new()
        .with_prompt("Are you using PipeWire? (recommended)")
        .default(true)
        .interact()
        .unwrap();

    if pipewire {
        packages.push("pipewire-pulse");
    }

    pacman_install(&packages);

    if is_systemd_running() {
        Command::new("sudo")
            .args(["systemctl", "enable", "--now", "bluetooth"])
            .status()
            .expect("Failed to enable bluetooth service");
    } else {
        ui::warn("Systemd not running, skipping service enable.");
        println!("  Run manually: sudo systemctl enable --now bluetooth");
    }

    ui::success("Bluetooth installed!");
}