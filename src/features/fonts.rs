use crate::system::{is_installed, pacman_install};
use crate::ui;

const FONTS: [(&str, &[&str]); 10] = [
    ("JetBrains Mono", &["ttf-jetbrains-mono-nerd"]),
    ("Fira Code",      &["ttf-firacode-nerd"]),
    ("Hack",           &["ttf-hack-nerd"]),
    ("Iosevka",        &["ttf-iosevka-nerd"]),
    ("Cascadia Code",  &["ttf-cascadia-code-nerd"]),
    ("Meslo",          &["ttf-meslo-nerd"]),
    ("Ubuntu",         &["ttf-ubuntu-nerd"]),
    ("Roboto Mono",    &["ttf-roboto-mono-nerd"]),
    ("Victor Mono",    &["ttf-victor-mono-nerd"]),
    ("Inconsolata",    &["ttf-inconsolata-nerd"]),
];

pub fn done() -> bool {
    FONTS.iter().any(|(_, packages)| is_installed(packages))
}

pub fn run() {
    let packages = ui::pick_packages("Select fonts to install", &FONTS);
    if packages.is_empty() {
        return;
    }

    pacman_install(&packages);
    ui::success("Fonts installed!");
}
