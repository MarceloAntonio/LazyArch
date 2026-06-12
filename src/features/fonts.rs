use dialoguer::MultiSelect;
use crate::system::pacman::pacman_install;
use crate::ui;

pub fn fonts_installer() {
    let fonts: Vec<(&str, &str)> = vec![
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

    let names: Vec<&str> = fonts.iter().map(|(name, _)| *name).collect();

    let selected = MultiSelect::new()
        .with_prompt("Select fonts to install")
        .items(&names)
        .interact()
        .unwrap();

    if selected.is_empty() {
        println!("Nothing selected, skipping...");
        return;
    }

    for idx in selected {
        let (name, package) = fonts[idx];
        ui::info(&format!("Installing {}...", name));
        pacman_install(&[package]);
    }

    ui::success("Fonts installed!");
}