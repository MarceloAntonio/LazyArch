use std::process::Command;
use dialoguer::{Confirm};
use crate::system::pacman::pacman_install;


pub fn bluetooth_setup() {
    let mut bluetooth_packages = vec!["bluez","bluez-utils","pipewire-pulse"];


            let confirmation = Confirm::new()
        .with_prompt("Do you want to install GUI?")
        .default(true) 
        .interact()
        .unwrap();

        if confirmation{
            bluetooth_packages.push("blueman")
        }
    pacman_install(&bluetooth_packages);

        Command::new("sudo")
            .args(["systemctl", "enable", "--now", "bluetooth"])
            .status()
            .unwrap();



        println!("Bluetooth installed!");

}