use std::os::unix::fs::MetadataExt;
use std::process::exit;

mod features;
mod menu;
mod system;
mod ui;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let arg = std::env::args().nth(1);

    match arg.as_deref() {
        Some("-v" | "--version") => return println!("lazy-arch {VERSION}"),
        Some("-h" | "--help") => return menu::print_help(),
        _ => {}
    }

    // Checked before any feature flag runs, not just the menu.
    if !system::is_arch_based() {
        ui::error("This system is not Arch-based, LazyArch only supports Arch Linux and its derivatives.");
        exit(1);
    }
    if std::fs::metadata("/proc/self").is_ok_and(|m| m.uid() == 0) {
        ui::error("Don't run LazyArch as root or with sudo, it asks for sudo when needed.");
        exit(1);
    }

    match arg.as_deref() {
        None => {
            ui::banner();
            menu::main_menu();
        }
        Some("--first-setup") => menu::first_setup(),
        Some(flag) => match menu::FEATURES.iter().find(|f| f.flag == flag) {
            Some(f) => f.run_checked(),
            None => {
                ui::error(&format!("Unknown option: {flag}"));
                println!("Run 'lazy-arch --help' for usage.");
                exit(1);
            }
        },
    }
}
