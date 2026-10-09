use std::fs;
use std::process::{Command, Stdio};

pub fn is_arch_based() -> bool {
    let Ok(content) = fs::read_to_string("/etc/os-release") else {
        return false;
    };

    for line in content.lines() {
        if let Some(value) = line.strip_prefix("ID=").or_else(|| line.strip_prefix("ID_LIKE=")) {
            let value = value.trim_matches('"').to_lowercase();
            if value.split_whitespace().any(|v| v == "arch") {
                return true;
            }
        }
    }

    false
}

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
