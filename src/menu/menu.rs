use dialoguer::{Select};

use crate::features::aur::install_aur;
use crate::features::shell::change_shell;
use crate::features::lazyvim::install_lazy_vim;
use crate::features::mirrors::update_mirrors;
use crate::features::gaming::gaming_setup;
use crate::features::docker::install_docker;



pub fn main_menu(){
    loop {
        let choices = vec!["Install AUR", "Change shell", "Install LazyVim", "Gaming Setup","Docker setup","Update Mirrors","Exit"];
        let selection = Select::new()
        .with_prompt("\nSelect an option:")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();
    
match choices[selection] {
        "Install AUR" => install_aur(),
        "Change shell" => change_shell(),
        "Install LazyVim" => install_lazy_vim(),
        "Docker setup" => install_docker(),
        "Update Mirrors" => update_mirrors(),
        "Gaming Setup" => gaming_setup(),
        "Exit" => std::process::exit(0),
        _ => println!("Critical error: unrecognized option."),
    }
}
    }






