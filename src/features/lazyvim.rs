use std::process::Command;
use dialoguer::{Confirm};
use crate::system::pacman::pacman_install;
use crate::features::setup_cfg_lazyvim::setup_cfg_lazyvim;



pub fn install_lazy_vim() {
    let home = std::env::var("HOME").expect("HOME not set");
    let nvim_path = format!("{}/.config/nvim", home);
    let nvim_git = format!("{}/.config/nvim/.git", home);

    let confirmation = Confirm::new()
        .with_prompt("Do you want to add a customization?")
        .default(true) 
        .interact()
        .unwrap();


    println!("\n\n# Installing dependencies #\n\n");
    pacman_install(&["nvim", "git"]);

    println!("\n\n# Cloning repository and doing installation #\n\n");
    Command::new("git")
    .args(["clone", "https://github.com/LazyVim/starter", &nvim_path])
    .status()
    .expect("Failed to clone LazyVim");


    if confirmation{
    println!("\n\n# installing customization #\n\n");
    setup_cfg_lazyvim();
    }

    Command::new("nvim")
        .args(["--headless","+Lazy!","sync","+qa"])
        .status()
        .expect("Failed to clean cache");
    
    println!("\n\n# Cleaning cache #\n\n");
    Command::new("rm")
        .args(["-rf", &nvim_git])
        .status()
        .expect("Failed to clean cache");





    println!("\n\n# Lazy vim installed successfully #\n\n");
}