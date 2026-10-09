use crate::system::{is_enabled, pacman_install, sudo};
use crate::ui;

pub fn done() -> bool {
    is_enabled("cups.service")
}

pub fn run() {
    pacman_install(&["cups", "system-config-printer"]);
    sudo(&["systemctl", "enable", "--now", "cups.service"]);

    ui::success("Printing ready! Add your printer in 'Print Settings'.");
    println!("  Tip: some printers need drivers, e.g. 'hplip' (HP) or 'gutenprint'.");
}
