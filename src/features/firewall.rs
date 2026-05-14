use std::process::Command;
use dialoguer::{Confirm, Select};
use crate::system::pacman::pacman_install;
use crate::system::is_systemd_running::is_systemd_running;

pub fn firewall_setup() {
    pacman_install(&["ufw"]);

        if !is_systemd_running() {
        println!("  Systemd not running (are you in a container?).");
        println!("   UFW requires kernel permissions unavailable in containers.");
        println!("   Run LazyArch on a real Arch system to configure the firewall.");
        return;
    }
    
    println!("==> Setting default rules...");
    Command::new("sudo").args(["ufw", "default", "deny", "incoming"]).status().unwrap();
    Command::new("sudo").args(["ufw", "default", "allow", "outgoing"]).status().unwrap();

  
    let allow_ssh = Confirm::new()
        .with_prompt("Allow SSH (port 22)?")
        .default(true)
        .interact()
        .unwrap();

    if allow_ssh {
        Command::new("sudo").args(["ufw", "allow", "ssh"]).status().unwrap();
    }


    let profiles = vec![
        "Personal (SSH only)",
        "Developer (SSH + HTTP + HTTPS)",
        "Custom (choose ports manually)",
    ];

    let profile = Select::new()
        .with_prompt("Select a firewall profile")
        .items(&profiles)
        .default(0)
        .interact()
        .unwrap();

    match profile {
        1 => {
            Command::new("sudo").args(["ufw", "allow", "80"]).status().unwrap();
            Command::new("sudo").args(["ufw", "allow", "443"]).status().unwrap();
            println!("==> HTTP and HTTPS allowed.");
        }
        2 => {
            let port: String = dialoguer::Input::new()
                .with_prompt("Enter port to allow (ex: 8080)")
                .interact_text()
                .unwrap();
            Command::new("sudo").args(["ufw", "allow", &port]).status().unwrap();
            println!("==> Port {} allowed.", port);
        }
        _ => {}
    }

    println!("==> Enabling firewall...");
    Command::new("sudo").args(["ufw", "--force", "enable"]).status().unwrap();

    Command::new("sudo").args(["ufw", "status", "verbose"]).status().unwrap();

    println!("✓ Firewall configured!");
}