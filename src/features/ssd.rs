use crate::system::sudo;
use crate::ui;

pub fn run() {
    ui::info("Running TRIM...");
    sudo(&["fstrim", "-av"]);

    ui::info("Enabling fstrim.timer...");
    sudo(&["systemctl", "enable", "--now", "fstrim.timer"]);

    ui::success("SSD setup done! TRIM will run automatically every week.");
}
