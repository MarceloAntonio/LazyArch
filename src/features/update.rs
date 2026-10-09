use crate::features::aur;
use crate::system::{self, is_installed, sudo};
use crate::ui;

pub fn run() {
    ui::info("Updating official packages...");
    sudo(&["pacman", "-Syu"]);

    if let Some(helper) = aur::installed_helper() {
        ui::info(&format!("Updating AUR packages with {helper}..."));
        system::run(helper, &["-Sua"]);
    }

    if is_installed(&["flatpak"]) {
        ui::info("Updating Flatpak apps...");
        system::run("flatpak", &["update"]);
    }

    ui::success("System up to date!");
}
