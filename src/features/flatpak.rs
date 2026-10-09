use std::process::Command;
use crate::system::{pacman_install, sudo};
use crate::ui;

pub fn done() -> bool {
    Command::new("flatpak")
        .args(["remotes", "--columns=name"])
        .output()
        .is_ok_and(|o| String::from_utf8_lossy(&o.stdout).lines().any(|l| l.trim() == "flathub"))
}

pub fn run() {
    pacman_install(&["flatpak"]);

    ui::info("Adding Flathub...");
    sudo(&["flatpak", "remote-add", "--if-not-exists", "flathub", "https://dl.flathub.org/repo/flathub.flatpakrepo"]);

    ui::success("Flatpak ready! Log out and back in so Flatpak apps show up in your app menu.");
}
