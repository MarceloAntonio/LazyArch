use std::process::Command;
use dialoguer::{Confirm, Input};
use crate::system::pacman::pacman_install;
use crate::ui;

pub fn git_setup() {
    ui::info("Git Configuration\n");

    pacman_install(&["git"]);

    let name: String = Input::new()
        .with_prompt("Git user name")
        .interact_text()
        .unwrap();

    Command::new("git")
        .args(["config", "--global", "user.name", &name])
        .status()
        .expect("Failed to set git user.name");

    let email: String = Input::new()
        .with_prompt("Git user email")
        .interact_text()
        .unwrap();

    Command::new("git")
        .args(["config", "--global", "user.email", &email])
        .status()
        .expect("Failed to set git user.email");

    ui::success("Git configured!");

    let gen_ssh = Confirm::new()
        .with_prompt("Generate SSH key? (ed25519)")
        .default(true)
        .interact()
        .unwrap();

    if gen_ssh {
        let home = std::env::var("HOME").expect("HOME not set");
        let key_path = format!("{}/.ssh/id_ed25519", home);

        Command::new("ssh-keygen")
            .args(["-t", "ed25519", "-C", &email, "-f", &key_path, "-N", ""])
            .status()
            .expect("Failed to generate SSH key");

        ui::success("SSH key generated!");
        println!("\n  Add to GitHub/GitLab:");
        println!("  cat {}.pub\n", key_path);
    }
}
