use dialoguer::{Select};

use crate::features::aur::install_aur;
use crate::features::shell::change_shell;
use crate::features::lazyvim::install_lazy_vim;
use crate::features::mirrors::update_mirrors;
use crate::features::gaming::gaming_setup;
use crate::features::docker::install_docker;
use crate::features::languages::language_installer;
use crate::features::pacman_cfg::pacman_cfg;
use crate::features::fonts::fonts_installer;
use crate::features::bluetooth::bluetooth_setup;



pub fn main_menu(){
    loop {
        let choices = vec!["Pacman Configuration","Install AUR","Bluetooth Setup","Install Nerd Fonts", "Change shell", "Install LazyVim", "GPU Drivers/Gaming Setup","Docker setup","Update Mirrors","Language Installer","Exit"];
        let selection = Select::new()
        .with_prompt("\nSelect an option:")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();
    
match choices[selection] {
        "Pacman Configuration" => pacman_cfg(),
        "Install AUR" => install_aur(),
        "Change shell" => change_shell(),
        "Install LazyVim" => install_lazy_vim(),
        "Bluetooth Setup" => bluetooth_setup(),
        "Install Nerd Fonts" => fonts_installer(),
        "Docker setup" => install_docker(),
        "Update Mirrors" => update_mirrors(),
        "GPU Drivers/Gaming Setup" => gaming_setup(),
        "Language Installer" => language_installer(),
        "Exit" => std::process::exit(0),
        _ => println!("Critical error: unrecognized option."),
    }
}
    }






