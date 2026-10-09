use crate::system::{is_enabled, sudo};
use crate::ui;

pub fn done() -> bool {
    is_enabled("fstrim.timer")
}

pub fn run() {
    ui::info("Running TRIM...");
    sudo(&["fstrim", "-av"]);

    ui::info("Enabling fstrim.timer...");
    sudo(&["systemctl", "enable", "--now", "fstrim.timer"]);

    ui::success("SSD setup done! TRIM will run automatically every week.");
}
