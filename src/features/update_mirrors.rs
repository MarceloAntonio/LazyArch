use std::process::Command;
use dialoguer::Select;
use crate::system::pacman::pacman_install;

pub fn update_mirrors() {
    
    pacman_install(&["reflector"]);

    let countries = vec![
        "Brazil",
        "United States", 
        "Germany",
        "France",
        "Japan",
        "Australia",
    ];

    let idx = Select::new()
        .with_prompt("Select your country for mirror optimization")
        .items(&countries)
        .default(0)
        .interact()
        .unwrap();

    let country = countries[idx];

    println!("==> Updating mirrors for {}...", country);

    Command::new("sudo")
        .args([
            "reflector",
            "--country", country,
            "--age", "12",        
            "--protocol", "https",
            "--sort", "rate",     
            "--save", "/etc/pacman.d/mirrorlist",
        ])
        .status()
        .expect("Failed to run reflector");

    
    Command::new("sudo")
        .args(["pacman", "-Sy"])
        .status()
        .expect("Failed to update pacman database");

    println!("✓ Mirrors updated successfully!");
}