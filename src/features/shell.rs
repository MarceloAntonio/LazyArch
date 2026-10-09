use std::fs;
use std::path::Path;
use dialoguer::{Confirm, Select};
use crate::system::{self, pacman_install};
use crate::ui;

/// Arch's default login shell is bash, anything else means it was changed.
pub fn done() -> bool {
    std::env::var("SHELL").is_ok_and(|s| !s.ends_with("/bash"))
}

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

    if matches!(*name, "Bash" | "Zsh" | "Fish") {
        let pretty = Confirm::new()
            .with_prompt("Make it pretty? (Starship prompt, plus suggestions and highlighting for Zsh)")
            .default(true)
            .interact()
            .unwrap();

        if pretty {
            prettify(name);
        }
    }
}

fn prettify(shell: &str) {
    let (packages, rc, lines): (&[&str], &str, &[&str]) = match shell {
        "Zsh" => (
            &["starship", "zsh-autosuggestions", "zsh-syntax-highlighting"],
            ".zshrc",
            &[
                "source /usr/share/zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh",
                "eval \"$(starship init zsh)\"",
                // Must be sourced last.
                "source /usr/share/zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh",
            ],
        ),
        // Fish has suggestions and highlighting built in.
        "Fish" => (&["starship"], ".config/fish/config.fish", &["starship init fish | source"]),
        _ => (&["starship"], ".bashrc", &["eval \"$(starship init bash)\""]),
    };

    pacman_install(packages);

    let home = std::env::var("HOME").expect("HOME not set");
    let rc = Path::new(&home).join(rc);
    let content = append_missing(fs::read_to_string(&rc).unwrap_or_default(), lines);

    if let Some(dir) = rc.parent() {
        fs::create_dir_all(dir).expect("Failed to create config directory");
    }
    fs::write(&rc, content).expect("Failed to write shell config");

    ui::success(&format!("Updated {}", rc.display()));
    println!("  Tip: Starship icons need a Nerd Font (Desktop > Install Nerd Fonts) set in your terminal.");
}

/// Appends each line not already in `content`, so running twice changes nothing.
fn append_missing(mut content: String, lines: &[&str]) -> String {
    for line in lines {
        if !content.lines().any(|l| l.trim() == *line) {
            if !content.is_empty() && !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(line);
            content.push('\n');
        }
    }
    content
}

#[cfg(test)]
mod tests {
    use super::append_missing;

    #[test]
    fn append_missing_is_idempotent() {
        let lines = ["a", "b"];
        let once = append_missing("x".to_string(), &lines);
        assert_eq!(once, "x\na\nb\n");
        assert_eq!(append_missing(once.clone(), &lines), once);
    }
}
