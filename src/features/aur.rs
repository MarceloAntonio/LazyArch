use std::process::Command;
use crate::system::pacman::pacman_install;

pub fn install_aur() {
    let install_dir = "/tmp/yay";

    println!("\n\n# Installing dependencies #\n\n");
    pacman_install(&["base-devel", "git"]);

    println!("\n\n# Cloning repository #\n\n");
    Command::new("git")
        .args(["clone", "https://aur.archlinux.org/yay.git", install_dir])
        .status()
        .expect("Failed to clone yay");

    println!("\n\n# Running the initial installation #\n\n");
    Command::new("makepkg")
        .args(["-si", "--noconfirm"])
        .current_dir(install_dir)
        .status()
        .expect("Failed to run makepkg");

    println!("\n\n# Cleaning cache #\n\n");
    Command::new("sudo")
        .args(["rm", "-rf", install_dir])
        .status()
        .expect("Failed to clean cache");
}