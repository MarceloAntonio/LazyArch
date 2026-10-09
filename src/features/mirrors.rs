use crate::system::{pacman_install, sudo};
use crate::ui;

pub fn run() {
    pacman_install(&["reflector"]);

    ui::info("Finding the fastest up-to-date mirrors...");
    sudo(&[
        "reflector",
        "--latest", "20",
        "--protocol", "https",
        "--sort", "rate",
        "--save", "/etc/pacman.d/mirrorlist",
    ]);

    // -Syu, never -Sy: a sync without upgrade is an unsupported partial upgrade.
    sudo(&["pacman", "-Syu"]);

    ui::success("Mirrors updated!");
}
