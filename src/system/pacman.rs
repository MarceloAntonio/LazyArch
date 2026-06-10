use std::process::{Command, Stdio};

pub fn pacman_install(packages: &[&str]) {
    let missing: Vec<&str> = packages
        .iter()
        .filter(|&&p| {
            !Command::new("pacman")
                .args(["-Qi", p])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .map(|s| s.success())
                .unwrap_or(false)
        })
        .copied()
        .collect();

    if missing.is_empty() {
        println!("{} already installed, skipping...", packages.join(", "));
        return;
    }

    Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm", "--needed"])
        .args(&missing)
        .status()
        .expect("Failed to run pacman");
}