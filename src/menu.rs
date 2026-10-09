use dialoguer::{MultiSelect, Select};

use crate::features::*;
use crate::ui;

pub struct Feature {
    pub flag: &'static str,
    pub name: &'static str,
    pub run: fn(),
    pub first_setup: bool,
}

// Single source of truth: menu, CLI flags, --help and First Setup all come from here.
pub const FEATURES: &[Feature] = &[
    // Base System
    Feature { flag: "--pacman",      name: "Pacman Configuration",     run: pacman_config::run, first_setup: true },
    Feature { flag: "--mirrors",     name: "Update Mirrors",           run: mirrors::run,       first_setup: true },
    Feature { flag: "--aur",         name: "Install AUR Helper",       run: aur::run,           first_setup: true },
    // Hardware
    Feature { flag: "--gaming",      name: "GPU Drivers/Gaming Setup", run: gaming::run,        first_setup: true },
    Feature { flag: "--bluetooth",   name: "Bluetooth Setup",          run: bluetooth::run,     first_setup: true },
    Feature { flag: "--ssd",         name: "SSD Trim Activation",      run: ssd::run,           first_setup: true },
    // Desktop
    Feature { flag: "--fonts",       name: "Install Nerd Fonts",       run: fonts::run,         first_setup: true },
    Feature { flag: "--shell",       name: "Change Shell",             run: shell::run,         first_setup: true },
    // Dev
    Feature { flag: "--languages",   name: "Language Installer",       run: languages::run,     first_setup: false },
    Feature { flag: "--docker",      name: "Docker Setup",             run: docker::run,        first_setup: false },
    Feature { flag: "--git",         name: "Git Setup",                run: git::run,           first_setup: true },
    // Maintenance
    Feature { flag: "--maintenance", name: "System Maintenance",       run: maintenance::run,   first_setup: false },
];

pub fn print_help() {
    println!("lazy-arch {} — Automate your Arch Linux setup\n", crate::VERSION);
    println!("Usage: lazy-arch [OPTION]\n");
    println!("Options:");
    println!("  -v, --version       Show version");
    println!("  -h, --help          Show this help");
    println!("  --first-setup       Run the first setup wizard");
    for f in FEATURES {
        println!("  {:<20}{}", f.flag, f.name);
    }
}

pub fn first_setup() {
    let steps: Vec<&Feature> = FEATURES.iter().filter(|f| f.first_setup).collect();
    let names: Vec<&str> = steps.iter().map(|f| f.name).collect();

    let selected = MultiSelect::new()
        .with_prompt("Select what to run in First Setup")
        .items(&names)
        .defaults(&vec![true; steps.len()])
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
    items.extend(FEATURES.iter().map(|f| f.name));
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
            i if i <= FEATURES.len() => (FEATURES[i - 1].run)(),
            _ => return,
        }
    }
}
