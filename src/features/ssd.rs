use std::process::Command;
use crate::system::is_systemd_running::is_systemd_running;
use crate::ui;

pub fn ssd_setup() {
    ui::info("Running TRIM...");
    Command::new("sudo")
        .args(["fstrim", "-av"])
        .status()
        .expect("Failed to run fstrim");

    if is_systemd_running() {
        ui::info("Enabling fstrim.timer...");
        Command::new("sudo")
            .args(["systemctl", "enable", "--now", "fstrim.timer"])
            .status()
            .expect("Failed to enable fstrim.timer");
    } else {
        ui::warn("Systemd not running, skipping timer setup.");
        println!("  Run manually: sudo systemctl enable --now fstrim.timer");
    }

    ui::success("SSD setup done! TRIM will run automatically every week.");
}