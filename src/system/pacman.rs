use std::process::Command;

pub fn pacman_install(packages: &[&str]) {
  
    let missing: Vec<&str> = packages
        .iter()
        .filter(|&&p| which::which(p).is_err())
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