use dialoguer::MultiSelect;
use crate::system::pacman::pacman_install;

pub fn language_installer() {
    let languages = vec!["Node.js", "Go", "Python", "Java", "Rust", "PHP", "C/C++"];

    let selected = MultiSelect::new()
        .with_prompt("Select languages to install")
        .items(&languages)
        .interact()
        .unwrap();

    if selected.is_empty() {
        println!("Nothing selected, skipping...");
        return;
    }

    for idx in &selected {
        match idx {
            0 => {
                println!("==> Installing Node.js...");
                pacman_install(&["nodejs", "npm"]);
            }
            1 => {
                println!("==> Installing Go...");
                pacman_install(&["go"]);
            }
            2 => {
                println!("==> Installing Python...");
                pacman_install(&["python", "python-pip", "python-virtualenv"]);
            }
            3 => {
                println!("==> Installing Java...");
                pacman_install(&["jdk-openjdk", "jre-openjdk", "maven"]);
            }
            4 => {
                println!("==> Installing Rust...");
                pacman_install(&["rust","cargo"]);
            }
            5 => {
                println!("==> Installing PHP...");
                pacman_install(&["php", "php-fpm", "composer"]);
            }
            6 => {
                println!("==> Installing C/C++...");
                pacman_install(&["gcc", "g++", "gdb", "cmake", "make", "clang"]);
            }
            _ => {}
        }
    }

    println!("\n✓ Done! Languages installed successfully.");
}