use dialoguer::MultiSelect;
use crate::system::{is_installed, pacman_install};
use crate::ui;

const FONTS: [(&str, &str); 10] = [
    ("JetBrains Mono", "ttf-jetbrains-mono-nerd"),
    ("Fira Code",      "ttf-firacode-nerd"),
    ("Hack",           "ttf-hack-nerd"),
    ("Iosevka",        "ttf-iosevka-nerd"),
    ("Cascadia Code",  "ttf-cascadia-code-nerd"),
    ("Meslo",          "ttf-meslo-nerd"),
    ("Ubuntu",         "ttf-ubuntu-nerd"),
    ("Roboto Mono",    "ttf-roboto-mono-nerd"),
    ("Victor Mono",    "ttf-victor-mono-nerd"),
    ("Inconsolata",    "ttf-inconsolata-nerd"),
];

pub fn done() -> bool {
    FONTS.iter().any(|(_, package)| is_installed(&[package]))
}

pub fn run() {
    let installed: Vec<bool> = FONTS.iter().map(|(_, package)| is_installed(&[package])).collect();
    let names: Vec<String> = FONTS
        .iter()
        .zip(&installed)
        .map(|((name, _), &inst)| if inst { format!("{name} (installed)") } else { name.to_string() })
        .collect();

    let selected = MultiSelect::new()
        .with_prompt("Select fonts to install")
        .items(&names)
        .defaults(&installed)
        .interact()
        .unwrap();

    if selected.is_empty() {
        println!("Nothing selected, skipping...");
        return;
    }

    let packages: Vec<&str> = selected.iter().map(|&idx| FONTS[idx].1).collect();
    pacman_install(&packages);

    ui::success("Fonts installed!");
}