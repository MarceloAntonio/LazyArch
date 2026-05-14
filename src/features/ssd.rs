use std::process::Command;
use crate::system::is_systemd_running::is_systemd_running;

pub fn sdd_setup() {

    println!("==> Running TRIM...");
    Command::new("sudo")
            .args(["fstrim","-av"])
            .status()
            .unwrap();
        

    if is_systemd_running() {
        println!("==> Enabling fstrim.timer...");
        Command::new("sudo")
        .args(["systemctl", "enable", "--now", "fstrim.timer"])
        .status()
        .unwrap();
    } 
    else {
        println!("Systemd not running, skipping timer setup.");
        println!("Run manually: sudo systemctl enable --now fstrim.timer");
    }

    println!("✓ SSD setup done! TRIM will run automatically every week.");

}