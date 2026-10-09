use std::process::Command;
use crate::ui;

pub fn run() {
    ui::info("Running TRIM...");
    Command::new("sudo")
        .args(["fstrim", "-av"])
        .status()
        .expect("Failed to run fstrim");

    ui::info("Enabling fstrim.timer...");
    Command::new("sudo")
        .args(["systemctl", "enable", "--now", "fstrim.timer"])
        .status()
        .expect("Failed to enable fstrim.timer");

    ui::success("SSD setup done! TRIM will run automatically every week.");
}