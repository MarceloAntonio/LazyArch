use std::process::Command;
use dialoguer::Select;
use crate::system::pacman::pacman_install;

pub fn update_mirrors() {
    // Garante que o reflector está instalado
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
            "--age", "12",        // mirrors atualizados nas últimas 12h
            "--protocol", "https",
            "--sort", "rate",     // ordena por velocidade
            "--save", "/etc/pacman.d/mirrorlist",
        ])
        .status()
        .expect("Failed to run reflector");

    // Atualiza o banco de dados do pacman com os novos mirrors
    Command::new("sudo")
        .args(["pacman", "-Sy"])
        .status()
        .expect("Failed to update pacman database");

    println!("✓ Mirrors updated successfully!");
}