use dialoguer::{Select};

use crate::features::aur::install_aur;
use crate::features::shell::change_shell;
use crate::features::lazyvim::install_lazy_vim;



pub fn main_menu(){
    loop {
        let choices = vec!["Install AUR", "Change shell", "Install LazyVim","Exit"];
        let selection = Select::new()
        .with_prompt("Select an option:")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();
    
match choices[selection] {
        "Install AUR" => install_aur(),
        "Change shell" => change_shell(),
        "Install LazyVim" => install_lazy_vim(),
        "Exit" => std::process::exit(0),
        _ => println!("Critical error: unrecognized option."),
    }
}
    }






