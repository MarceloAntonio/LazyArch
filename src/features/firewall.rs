use std::process::Command;
use dialoguer::{Confirm, Select};
use crate::system::pacman::pacman_install;
use crate::system::is_systemd_running::is_systemd_running;
use crate::ui;

pub fn firewall_setup() {
    pacman_install(&["ufw"]);

    if !is_systemd_running() {
        ui::warn("Systemd not running (are you in a container?).");
        println!("  UFW requires kernel permissions unavailable in containers.");
        println!("  Run LazyArch on a real Arch system to configure the firewall.");
        return;
    }

    ui::info("Setting default rules...");
    Command::new("sudo").args(["ufw", "default", "deny", "incoming"]).status().expect("Failed to set UFW rule");
    Command::new("sudo").args(["ufw", "default", "allow", "outgoing"]).status().expect("Failed to set UFW rule");

    let allow_ssh = Confirm::new()
        .with_prompt("Allow SSH (port 22)?")
        .default(true)
        .interact()
        .unwrap();

    if allow_ssh {
        Command::new("sudo").args(["ufw", "allow", "ssh"]).status().expect("Failed to allow SSH");
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
            Command::new("sudo").args(["ufw", "allow", "80"]).status().expect("Failed to allow port 80");
            Command::new("sudo").args(["ufw", "allow", "443"]).status().expect("Failed to allow port 443");
            ui::info("HTTP and HTTPS allowed.");
        }
        2 => {
            let port: String = dialoguer::Input::new()
                .with_prompt("Enter port to allow (ex: 8080)")
                .interact_text()
                .unwrap();
            Command::new("sudo").args(["ufw", "allow", &port]).status().expect("Failed to allow port");
            ui::info(&format!("Port {} allowed.", port));
        }
        _ => {}
    }

    ui::info("Enabling firewall...");
    Command::new("sudo").args(["ufw", "--force", "enable"]).status().expect("Failed to enable UFW");
    Command::new("sudo").args(["ufw", "status", "verbose"]).status().expect("Failed to show UFW status");

    ui::success("Firewall configured!");
}