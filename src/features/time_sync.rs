use std::process::Command;
use crate::system::sudo;
use crate::ui;

pub fn done() -> bool {
    Command::new("timedatectl")
        .args(["show", "--property=NTP", "--value"])
        .output()
        .is_ok_and(|o| String::from_utf8_lossy(&o.stdout).trim() == "yes")
}

pub fn run() {
    ui::info("Enabling automatic time sync (NTP)...");
    sudo(&["timedatectl", "set-ntp", "true"]);

    ui::success("Clock will stay in sync automatically.");
    println!("  Tip: dual booting and Windows shows the wrong time? Make Windows use UTC:");
    println!("  https://wiki.archlinux.org/title/System_time#UTC_in_Microsoft_Windows");
}
