use dialoguer::{Select};

use crate::features::{
    aur::install_aur,
    shell::change_shell,
    lazyvim::install_lazy_vim,
    mirrors::update_mirrors,
    gaming::gaming_setup,
    docker::install_docker,
    languages::language_installer,
    pacman::pacman_cfg,
    fonts::fonts_installer,
    bluetooth::bluetooth_setup,
    ssd::sdd_setup,
    firewall::firewall_setup
};



pub fn main_menu(){
    loop {
        let choices = vec!["Pacman Configuration","Install AUR","Bluetooth Setup","Install Nerd Fonts", "Change shell", "Install LazyVim", "GPU Drivers/Gaming Setup","Docker setup","Update Mirrors","Language Installer","Firewall activation","SSD trim activation","Exit"];

        let selection = Select::new()
        .with_prompt("\nSelect an option:")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();
    
match choices[selection] {
        "Pacman Configuration" => pacman_cfg(),
        "Install AUR" => install_aur(),
        "SSD trim activation" => sdd_setup(),
        "Firewall activation" => firewall_setup(),
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






