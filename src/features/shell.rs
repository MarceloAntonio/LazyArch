use dialoguer::{Select};
use crate::system::pacman::pacman_install;
use std::process::Command;

fn configuring_shell(shell: &str,shell_path: &str){
        println!("Switching to {}",shell);
        Command::new("chsh")
        .args(["-s",shell_path])
        .status()
        .expect("Failed to clone yay");
        println!("Shell changed successfully; restart or log back into your user");
}
 
pub fn change_shell(){
 let choices = vec!["Bash","Zsh", "Fish", "Back"];
 let selection = Select::new()
        .with_prompt("Select an option:")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();

        match choices[selection] {
        
        "Bash" =>{
                pacman_install(&["bash"]);
                configuring_shell("bash","/bin/bash")
        } 
        "Zsh" =>{
                pacman_install(&["zsh", "zsh-completions"]);
                configuring_shell("zsh", "/usr/bin/zsh")
        } 
        "Fish" =>{
                pacman_install(&["fish"]);
                configuring_shell("fish","/usr/bin/fish")
        } 
        
        "Back" => println!("returning to menu"),
        
        _ => println!("Critical error: unrecognized option."),
    }
}


   