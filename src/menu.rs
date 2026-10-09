use dialoguer::{Confirm, MultiSelect, Select};

use crate::features::*;
use crate::ui;

pub struct Feature {
    pub category: &'static str,
    pub flag: &'static str,
    pub name: &'static str,
    pub run: fn(),
    /// Checks the system: true if this is already installed/configured.
    pub done: fn() -> bool,
    pub first_setup: bool,
}

impl Feature {
    fn label(&self) -> String {
        if (self.done)() { format!("{} ✓", self.name) } else { self.name.to_string() }
    }

    /// Asks before redoing something that's already done.
    pub fn run_checked(&self) {
        let again = !(self.done)()
            || Confirm::new()
                .with_prompt(format!("{} is already done. Run it again?", self.name))
                .default(false)
                .interact()
                .unwrap();

        if again {
            (self.run)();
        }
    }
}

const BASE: &str = "Base System";
const HARDWARE: &str = "Hardware";
const DESKTOP: &str = "Desktop";
const DEV: &str = "Development";
const MAINTENANCE: &str = "Maintenance";

/// Submenu order in the main menu.
const CATEGORIES: [&str; 5] = [BASE, HARDWARE, DESKTOP, DEV, MAINTENANCE];

// Single source of truth: menu, CLI flags, --help and First Setup all come from here.
pub const FEATURES: &[Feature] = &[
    Feature { category: BASE,        flag: "--pacman",      name: "Pacman Configuration",     run: pacman_config::run, done: pacman_config::done, first_setup: true },
    Feature { category: BASE,        flag: "--mirrors",     name: "Update Mirrors",           run: mirrors::run,       done: mirrors::done,    first_setup: true },
    Feature { category: BASE,        flag: "--aur",         name: "Install AUR Helper",       run: aur::run,           done: aur::done,        first_setup: true },
    Feature { category: HARDWARE,    flag: "--gaming",      name: "GPU Drivers/Gaming Setup", run: gaming::run,        done: gaming::done,     first_setup: true },
    Feature { category: HARDWARE,    flag: "--bluetooth",   name: "Bluetooth Setup",          run: bluetooth::run,     done: bluetooth::done,  first_setup: true },
    Feature { category: HARDWARE,    flag: "--ssd",         name: "SSD Trim Activation",      run: ssd::run,           done: ssd::done,        first_setup: true },
    Feature { category: DESKTOP,     flag: "--fonts",       name: "Install Nerd Fonts",       run: fonts::run,         done: fonts::done,      first_setup: true },
    Feature { category: DESKTOP,     flag: "--shell",       name: "Change Shell",             run: shell::run,         done: shell::done,      first_setup: true },
    Feature { category: DEV,         flag: "--languages",   name: "Language Installer",       run: languages::run,     done: || false,         first_setup: false },
    Feature { category: DEV,         flag: "--docker",      name: "Docker Setup",             run: docker::run,        done: docker::done,     first_setup: false },
    Feature { category: DEV,         flag: "--git",         name: "Git Setup",                run: git::run,           done: git::done,        first_setup: true },
    Feature { category: MAINTENANCE, flag: "--maintenance", name: "System Maintenance",       run: maintenance::run,   done: || false,         first_setup: false },
];

pub fn print_help() {
    println!("lazy-arch {} — Automate your Arch Linux setup\n", crate::VERSION);
    println!("Usage: lazy-arch [OPTION]\n");
    println!("Options:");
    println!("  -v, --version       Show version");
    println!("  -h, --help          Show this help");
    println!("  --first-setup       Run the first setup wizard");
    for category in CATEGORIES {
        println!("\n{category}:");
        for f in FEATURES.iter().filter(|f| f.category == category) {
            println!("  {:<20}{}", f.flag, f.name);
        }
    }
}

pub fn first_setup() {
    let steps: Vec<&Feature> = FEATURES.iter().filter(|f| f.first_setup).collect();
    let names: Vec<String> = steps.iter().map(|f| f.label()).collect();
    // Already done steps start unchecked, tick them to run again.
    let pending: Vec<bool> = steps.iter().map(|f| !(f.done)()).collect();

    let selected = MultiSelect::new()
        .with_prompt("Select what to run in First Setup (✓ = already done)")
        .items(&names)
        .defaults(&pending)
        .interact()
        .unwrap();

    if selected.is_empty() {
        println!("Nothing selected, skipping...");
        return;
    }

    println!();
    ui::info("Starting First Setup...\n");

    for idx in selected {
        println!("\n=============================");
        ui::info(steps[idx].name);
        println!("=============================\n");
        (steps[idx].run)();
    }

    ui::success("First Setup complete! Reboot to apply all changes.");
}

pub fn main_menu() {
    let mut items = vec!["First Setup"];
    items.extend(CATEGORIES);
    items.push("Exit");

    loop {
        let selection = Select::new()
            .with_prompt("\nSelect an option")
            .items(&items)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => first_setup(),
            i if i <= CATEGORIES.len() => category_menu(CATEGORIES[i - 1]),
            _ => return,
        }
    }
}

fn category_menu(category: &str) {
    let features: Vec<&Feature> = FEATURES.iter().filter(|f| f.category == category).collect();

    loop {
        // Rebuilt every loop so ✓ reflects what was just done.
        let mut items: Vec<String> = features.iter().map(|f| f.label()).collect();
        items.push("Back".to_string());

        let selection = Select::new()
            .with_prompt(format!("\n{category}"))
            .items(&items)
            .default(0)
            .interact()
            .unwrap();

        match features.get(selection) {
            Some(f) => f.run_checked(),
            None => return,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_feature_is_reachable() {
        for f in FEATURES {
            assert!(CATEGORIES.contains(&f.category), "{} has a category missing from CATEGORIES", f.name);
            assert_eq!(FEATURES.iter().filter(|g| g.flag == f.flag).count(), 1, "duplicate flag {}", f.flag);
        }
    }
}
