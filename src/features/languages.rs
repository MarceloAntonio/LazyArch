use dialoguer::MultiSelect;
use crate::system::pacman::pacman_install;
use crate::ui;

pub fn language_installer() {
    let languages: Vec<(&str, Vec<&str>)> = vec![
        ("Node.js", vec!["nodejs", "npm"]),
        ("Go",      vec!["go"]),
        ("Python",  vec!["python", "python-pip", "python-virtualenv"]),
        ("Java",    vec!["jdk-openjdk", "maven"]),
        ("Rust",    vec!["rust", "cargo"]),
        ("PHP",     vec!["php", "php-fpm", "composer"]),
        ("C/C++",   vec!["gcc", "gdb", "cmake", "make", "clang"]),
        ("Ruby",    vec!["ruby"]),
        ("Elixir",  vec!["elixir"]),
        ("Zig",     vec!["zig"]),
        ("Lua",     vec!["lua", "luarocks"]),
        ("C#/.NET", vec!["dotnet-sdk"]),
    ];

    let names: Vec<&str> = languages.iter().map(|(name, _)| *name).collect();

    let selected = MultiSelect::new()
        .with_prompt("Select languages to install")
        .items(&names)
        .interact()
        .unwrap();

    if selected.is_empty() {
        println!("Nothing selected, skipping...");
        return;
    }

    for idx in selected {
        let (name, packages) = &languages[idx];
        ui::info(&format!("Installing {}...", name));
        pacman_install(packages);
    }

    ui::success("Languages installed!");
}