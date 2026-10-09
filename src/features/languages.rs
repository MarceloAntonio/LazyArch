use dialoguer::MultiSelect;
use crate::system::{is_installed, pacman_install};
use crate::ui;

const LANGUAGES: [(&str, &[&str]); 12] = [
    ("Node.js", &["nodejs", "npm"]),
    ("Go",      &["go"]),
    ("Python",  &["python", "python-pip", "python-virtualenv"]),
    ("Java",    &["jdk-openjdk", "maven"]),
    ("Rust",    &["rust"]),
    ("PHP",     &["php", "php-fpm", "composer"]),
    ("C/C++",   &["gcc", "gdb", "cmake", "make", "clang"]),
    ("Ruby",    &["ruby"]),
    ("Elixir",  &["elixir"]),
    ("Zig",     &["zig"]),
    ("Lua",     &["lua", "luarocks"]),
    ("C#/.NET", &["dotnet-sdk"]),
];

pub fn run() {
    let installed: Vec<bool> = LANGUAGES.iter().map(|(_, packages)| is_installed(packages)).collect();
    let names: Vec<String> = LANGUAGES
        .iter()
        .zip(&installed)
        .map(|((name, _), &inst)| if inst { format!("{name} (installed)") } else { name.to_string() })
        .collect();

    let selected = MultiSelect::new()
        .with_prompt("Select languages to install")
        .items(&names)
        .defaults(&installed)
        .interact()
        .unwrap();

    if selected.is_empty() {
        println!("Nothing selected, skipping...");
        return;
    }

    let packages: Vec<&str> = selected.iter().flat_map(|&idx| LANGUAGES[idx].1.iter().copied()).collect();
    pacman_install(&packages);

    ui::success("Languages installed!");
}