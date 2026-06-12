use std::process::Command;
use dialoguer::Select;
use crate::system::pacman::pacman_install;
use crate::ui;

fn set_shell(shell: &str, shell_path: &str) {
    ui::info(&format!("Switching to {}...", shell));
    Command::new("chsh")
        .args(["-s", shell_path])
        .status()
        .expect("Failed to change shell");
    ui::success("Shell changed! Restart or log back in to apply.");
}

pub fn change_shell() {
    let choices = vec!["Bash", "Zsh", "Fish", "Nushell", "Elvish", "Tcsh", "Back"];

    let selection = Select::new()
        .with_prompt("Select a shell")
        .items(&choices)
        .default(0)
        .interact()
        .unwrap();

    match choices[selection] {
        "Bash" => {
            pacman_install(&["bash"]);
            set_shell("bash", "/bin/bash");
        }
        "Zsh" => {
            pacman_install(&["zsh", "zsh-completions"]);
            set_shell("zsh", "/usr/bin/zsh");
        }
        "Fish" => {
            pacman_install(&["fish"]);
            set_shell("fish", "/usr/bin/fish");
        }
        "Nushell" => {
            pacman_install(&["nushell"]);
            set_shell("nu", "/usr/bin/nu");
        }
        "Elvish" => {
            pacman_install(&["elvish"]);
            set_shell("elvish", "/usr/bin/elvish");
        }
        "Tcsh" => {
            pacman_install(&["tcsh"]);
            set_shell("tcsh", "/usr/bin/tcsh");
        }
        "Back" => {}
        _ => ui::error("Unrecognized option."),
    }
}