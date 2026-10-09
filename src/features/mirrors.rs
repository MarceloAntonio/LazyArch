use crate::system::{pacman_install, sudo};
use crate::ui;

const MIRRORLIST: &str = "/etc/pacman.d/mirrorlist";

/// reflector writes its name in the mirrorlist header.
pub fn done() -> bool {
    std::fs::read_to_string(MIRRORLIST).is_ok_and(|c| c.to_lowercase().contains("reflector"))
}

pub fn run() {
    pacman_install(&["reflector"]);

    ui::info("Finding the fastest up-to-date mirrors...");
    sudo(&[
        "reflector",
        "--latest", "20",
        "--protocol", "https",
        "--sort", "rate",
        "--save", MIRRORLIST,
    ]);

    // -Syu, never -Sy: a sync without upgrade is an unsupported partial upgrade.
    sudo(&["pacman", "-Syu"]);

    ui::success("Mirrors updated!");
}
