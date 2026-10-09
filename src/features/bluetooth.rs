use std::process::Command;
use dialoguer::Confirm;
use crate::system::pacman::pacman_install;
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

    Command::new("sudo")
        .args(["systemctl", "enable", "--now", "bluetooth"])
        .status()
        .expect("Failed to enable bluetooth service");

    ui::success("Bluetooth installed!");
}