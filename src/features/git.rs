use dialoguer::{Confirm, Input};
use crate::system::{self, pacman_install, succeeds};
use crate::ui;

pub fn done() -> bool {
    succeeds("git", &["config", "--global", "user.name"])
}

pub fn run() {
    ui::info("Git Configuration\n");

    pacman_install(&["git"]);

    let name: String = Input::new()
        .with_prompt("Git user name")
        .interact_text()
        .unwrap();
    system::run("git", &["config", "--global", "user.name", &name]);

    let email: String = Input::new()
        .with_prompt("Git user email")
        .interact_text()
        .unwrap();
    system::run("git", &["config", "--global", "user.email", &email]);

    ui::success("Git configured!");

    let gen_ssh = Confirm::new()
        .with_prompt("Generate SSH key? (ed25519)")
        .default(true)
        .interact()
        .unwrap();

    if gen_ssh {
        let home = std::env::var("HOME").expect("HOME not set");
        let key_path = format!("{}/.ssh/id_ed25519", home);

        // No -N: ssh-keygen asks for a passphrase (Enter for none) and before overwriting a key.
        system::run("ssh-keygen", &["-t", "ed25519", "-C", &email, "-f", &key_path]);

        ui::success("SSH key generated!");
        println!("\n  Add to GitHub/GitLab:");
        println!("  cat {}.pub\n", key_path);
    }
}
