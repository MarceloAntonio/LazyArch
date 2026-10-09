use std::fs;
use std::path::Path;
use dialoguer::Select;
use crate::system::{self, pacman_install};
use crate::ui;

const HELPERS: [&str; 2] = ["yay", "paru"];

fn is_installed(helper: &str) -> bool {
    Path::new("/usr/bin").join(helper).exists()
}

pub fn done() -> bool {
    HELPERS.iter().any(|h| is_installed(h))
}

pub fn run() {

    let idx = Select::new()
        .with_prompt("Select an AUR helper")
        .items(&HELPERS)
        .default(0)
        .interact()
        .unwrap();

    let helper = HELPERS[idx];

    if is_installed(helper) {
        ui::success(&format!("{helper} already installed, skipping..."));
        return;
    }

    ui::info("Installing dependencies...");
    pacman_install(&["base-devel", "git"]);

    let build_dir = std::env::temp_dir().join(format!("lazyarch-{helper}"));
    let build_dir_str = build_dir.to_string_lossy();
    // A leftover dir from a failed run would make `git clone` fail.
    let _ = fs::remove_dir_all(&build_dir);

    ui::info(&format!("Cloning {helper}..."));
    system::run("git", &["clone", &format!("https://aur.archlinux.org/{helper}.git"), &build_dir_str]);

    ui::info(&format!("Building and installing {helper}..."));
    system::run("makepkg", &["-si", "-D", &build_dir_str]);

    let _ = fs::remove_dir_all(&build_dir);
    ui::success(&format!("{helper} installed!"));
}
