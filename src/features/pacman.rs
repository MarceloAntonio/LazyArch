use std::process::Command;
use crate::ui;

pub fn pacman_cfg() {
    let path = "/etc/pacman.conf";

    ui::info("Enabling Colors...");
    Command::new("sudo")
        .args(["sed", "-i", "s/#Color/Color/", path])
        .status()
        .expect("Failed to configure pacman");

    ui::info("Enabling parallel downloads...");
    Command::new("sudo")
        .args(["sed", "-i", "s/#ParallelDownloads = 5/ParallelDownloads = 5/", path])
        .status()
        .expect("Failed to configure pacman");

    ui::info("Enabling progress bar...");
    Command::new("sudo")
        .args(["sed", "-i", "s/^NoProgressBar/#NoProgressBar/", path])
        .status()
        .expect("Failed to configure pacman");

    ui::info("Adding ILoveCandy...");
    Command::new("sudo")
        .args(["sed", "-i", "/^Color$/a ILoveCandy", path])
        .status()
        .expect("Failed to configure pacman");

    ui::success("Pacman configured!");
}