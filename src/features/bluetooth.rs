use dialoguer::Confirm;
use crate::system::{is_enabled, pacman_install, sudo};
use crate::ui;

pub fn done() -> bool {
    is_enabled("bluetooth")
}

pub fn run() {
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
    sudo(&["systemctl", "enable", "--now", "bluetooth"]);

    ui::success("Bluetooth installed!");
}
