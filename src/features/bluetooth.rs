use std::process::Command;
use dialoguer::{Confirm};
use crate::system::pacman::pacman_install;
use crate::system::is_systemd_running::is_systemd_running;


pub fn bluetooth_setup() {
        let mut bluetooth_packages = vec!["bluez","bluez-utils"];


        let gui = Confirm::new()
        .with_prompt("Install Blueman (GUI manager)?")
        .default(true) 
        .interact()
        .unwrap();

        if gui{
            bluetooth_packages.push("blueman")
        }

        let pipeware = Confirm::new()
        .with_prompt("Are you using PipeWire? (recommended)")
        .default(true) 
        .interact()
        .unwrap();

        if pipeware{
            bluetooth_packages.push("pipewire-pulse")
        }

        pacman_install(&bluetooth_packages);

        if is_systemd_running(){

            Command::new("sudo")
                .args(["systemctl", "enable", "--now", "bluetooth"])
                .status()
                .unwrap();
        }
        else{
            println!("Systemd not running, skipping service enable.");
            println!("Run manually: sudo systemctl enable --now bluetooth");
        }


        println!("Bluetooth installed!");

}