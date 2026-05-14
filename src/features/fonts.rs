use dialoguer::MultiSelect;
use crate::system::pacman::pacman_install;

pub fn fonts_installer() {
    let fonts: Vec<(&str, &str)> = vec![
        ("JetBrains Mono", "ttf-jetbrains-mono-nerd"),
        ("Fira Code",      "ttf-firacode-nerd"),
        ("Hack",           "ttf-hack-nerd"),
        ("Iosevka",        "ttf-iosevka-nerd"),
        ("Cascadia Code",  "ttf-cascadia-code-nerd"),
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
        println!("==> Installing {}...", name);
        pacman_install(&[package]);
    }

    println!("\n✓ Done! Fonts installed successfully.");
}