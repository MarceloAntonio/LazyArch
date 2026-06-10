use std::process::Command;
use crate::system::pacman::pacman_install;
use crate::ui;

pub fn install_aur() {
    let install_dir = "/tmp/yay";

    ui::info("Installing dependencies...");
    pacman_install(&["base-devel", "git"]);

    ui::info("Cloning yay repository...");
    Command::new("git")
        .args(["clone", "https://aur.archlinux.org/yay.git", install_dir])
        .status()
        .expect("Failed to clone yay");

    ui::info("Building and installing yay...");
    Command::new("makepkg")
        .args(["-si", "--noconfirm"])
        .current_dir(install_dir)
        .status()
        .expect("Failed to build yay");

    ui::info("Cleaning up...");
    Command::new("sudo")
        .args(["rm", "-rf", install_dir])
        .status()
        .expect("Failed to clean up");

    ui::success("yay installed!");
}