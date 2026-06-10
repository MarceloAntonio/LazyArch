use dialoguer::{MultiSelect, Select};

use crate::features::{
    aur::install_aur,
    bluetooth::bluetooth_setup,
    desktop::desktop_installer,
    docker::install_docker,
    firewall::firewall_setup,
    fonts::fonts_installer,
    gaming::gaming_setup,
    git::git_setup,
    languages::language_installer,
    lazyvim::install_lazy_vim,
    maintenance::maintenance_menu,
    mirrors::update_mirrors,
    pacman::pacman_cfg,
    shell::change_shell,
    ssd::ssd_setup,
};

use crate::ui;

pub fn first_setup() {
    let steps: Vec<(&str, fn())> = vec![
        ("Pacman Configuration", pacman_cfg),
        ("Update Mirrors",       update_mirrors),
        ("Install AUR",          install_aur),
        ("GPU Drivers/Gaming",   gaming_setup),
        ("Bluetooth Setup",      bluetooth_setup),
        ("SSD Trim",             ssd_setup),
        ("Desktop/WM Installer", desktop_installer),
        ("Install Nerd Fonts",   fonts_installer),
        ("Change Shell",         change_shell),
        ("Install LazyVim",      install_lazy_vim),
        ("Git Setup",            git_setup),
        ("Firewall",             firewall_setup),
    ];

    let names: Vec<&str> = steps.iter().map(|(name, _)| *name).collect();
    let defaults = vec![true; steps.len()];

    let selected = MultiSelect::new()
        .with_prompt("Select what to run in First Setup")
        .items(&names)
        .defaults(&defaults)
        .interact()
        .unwrap();

    if selected.is_empty() {
        println!("Nothing selected, skipping...");
        return;
    }

    println!();
    ui::info("Starting First Setup...\n");

    for idx in selected {
        let (name, func) = steps[idx];
        println!("\n=============================");
        ui::info(name);
        println!("=============================\n");
        func();
    }

    ui::success("First Setup complete! Reboot to apply all changes.");
}

pub fn main_menu() {
    loop {
        let choices = vec![
            // Base System
            "First Setup",
            "Pacman Configuration",
            "Update Mirrors",
            "Install AUR",
            // Hardware
            "GPU Drivers/Gaming Setup",
            "Bluetooth Setup",
            "SSD Trim Activation",
            // Desktop
            "Desktop/WM Installer",
            "Install Nerd Fonts",
            "Change Shell",
            // Dev
            "Install LazyVim",
            "Language Installer",
            "Docker Setup",
            "Git Setup",
            // Security
            "Firewall Activation",
            // Maintenance
            "System Maintenance",

            "Exit",
        ];

        let selection = Select::new()
            .with_prompt("\nSelect an option")
            .items(&choices)
            .default(0)
            .interact()
            .unwrap();

        match choices[selection] {
            "First Setup"              => first_setup(),
            "Pacman Configuration"     => pacman_cfg(),
            "Update Mirrors"           => update_mirrors(),
            "Install AUR"              => install_aur(),

            "GPU Drivers/Gaming Setup" => gaming_setup(),
            "Bluetooth Setup"          => bluetooth_setup(),
            "SSD Trim Activation"      => ssd_setup(),

            "Desktop/WM Installer"     => desktop_installer(),
            "Install Nerd Fonts"       => fonts_installer(),
            "Change Shell"             => change_shell(),

            "Install LazyVim"          => install_lazy_vim(),
            "Language Installer"       => language_installer(),
            "Docker Setup"             => install_docker(),
            "Git Setup"                => git_setup(),

            "Firewall Activation"      => firewall_setup(),

            "System Maintenance"       => maintenance_menu(),

            "Exit" => std::process::exit(0),
            _ => {
                ui::error(&format!("Unrecognized option: {}", choices[selection]));
                std::thread::sleep(std::time::Duration::from_secs(2));
            }
        }
    }
}
