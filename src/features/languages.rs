use crate::system::pacman_install;
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
    let packages = ui::pick_packages("Select languages to install", &LANGUAGES);
    if packages.is_empty() {
        return;
    }

    pacman_install(&packages);
    ui::success("Languages installed!");
}
