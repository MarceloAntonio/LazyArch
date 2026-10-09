use crate::ui;
use crate::menu::first_setup;
use crate::features::{
    aur::install_aur,
    bluetooth::bluetooth_setup,
    docker::install_docker,
    fonts::fonts_installer,
    gaming::gaming_setup,
    git::git_setup,
    languages::language_installer,
    maintenance::maintenance_menu,
    mirrors::update_mirrors,
    pacman::pacman_cfg,
    shell::change_shell,
    ssd::ssd_setup,
};

pub fn parse_args(args: &[String], version: &str) -> bool {
    if args.len() > 1 {
        match args[1].as_str() {
            "--first-setup" => { first_setup(); true }
            "--pacman" => { pacman_cfg(); true }
            "--mirrors" => { update_mirrors(); true }
            "--aur" => { install_aur(); true }
            "--gaming" => { gaming_setup(); true }
            "--bluetooth" => { bluetooth_setup(); true }
            "--ssd" => { ssd_setup(); true }
            "--fonts" => { fonts_installer(); true }
            "--shell" => { change_shell(); true }
            "--languages" => { language_installer(); true }
            "--docker" => { install_docker(); true }
            "--git" => { git_setup(); true }
            "--maintenance" => { maintenance_menu(); true }

            "--version" | "-v" => {
                println!("lazy-arch {}", version);
                true
            }
            "--help" | "-h" => {
                println!("lazy-arch {} — Automate your Arch Linux setup", version);
                println!();
                println!("Usage: lazy-arch [OPTIONS]");
                println!();
                println!("Options:");
                println!("  -v, --version       Show version");
                println!("  -h, --help          Show this help");
                println!("  --first-setup       Run the first setup wizard");
                println!("  --pacman            Configure Pacman");
                println!("  --mirrors           Update Mirrors");
                println!("  --aur               Install AUR helper");
                println!("  --gaming            GPU Drivers & Gaming Setup");
                println!("  --bluetooth         Bluetooth Setup");
                println!("  --ssd               SSD Trim Activation");
                println!("  --fonts             Install Nerd Fonts");
                println!("  --shell             Change Shell");
                println!("  --languages         Language Installer");
                println!("  --docker            Docker Setup");
                println!("  --git               Git Setup");
                println!("  --maintenance       System Maintenance");
                true
            }
            _ => {
                ui::error(&format!("Unknown option: {}", args[1]));
                println!("Run 'lazy-arch --help' for usage.");
                true
            }
        }
    } else {
        false
    }
}