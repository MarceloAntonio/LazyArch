use dialoguer::Select;
use crate::system::{self, pacman_install};
use crate::ui;

pub fn run() {
    // (name, packages, path)
    let shells: [(&str, &[&str], &str); 6] = [
        ("Bash",    &["bash"],                   "/usr/bin/bash"),
        ("Zsh",     &["zsh", "zsh-completions"], "/usr/bin/zsh"),
        ("Fish",    &["fish"],                   "/usr/bin/fish"),
        ("Nushell", &["nushell"],                "/usr/bin/nu"),
        ("Elvish",  &["elvish"],                 "/usr/bin/elvish"),
        ("Tcsh",    &["tcsh"],                   "/usr/bin/tcsh"),
    ];

    let mut names: Vec<&str> = shells.iter().map(|(name, _, _)| *name).collect();
    names.push("Back");

    let idx = Select::new()
        .with_prompt("Select a shell")
        .items(&names)
        .default(0)
        .interact()
        .unwrap();

    let Some((name, packages, path)) = shells.get(idx) else {
        return;
    };

    pacman_install(packages);
    ui::info(&format!("Switching to {name}..."));
    system::run("chsh", &["-s", path]);
    ui::success("Shell changed! Restart or log back in to apply.");
}
