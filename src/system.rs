use std::fs;
use std::path::Path;
use std::process::{exit, Command};

use dialoguer::Confirm;

use crate::ui;

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

/// Runs a command. On failure, asks whether to keep going and quits if not.
pub fn run(cmd: &str, args: &[&str]) {
    let error = match Command::new(cmd).args(args).status() {
        Ok(status) if status.success() => return,
        Ok(status) => format!("`{cmd} {}` failed ({status})", args.join(" ")),
        Err(e) => format!("Could not run `{cmd}`: {e}"),
    };

    ui::error(&error);
    let keep_going = Confirm::new()
        .with_prompt("Continue anyway? (No quits LazyArch)")
        .default(false)
        .interact()
        .unwrap_or(false);

    if !keep_going {
        exit(1);
    }
}

pub fn sudo(args: &[&str]) {
    run("sudo", args);
}

/// `--needed` skips what's already installed. No `--noconfirm`, so pacman
/// shows what it's about to install and asks first.
pub fn pacman_install(packages: &[&str]) {
    sudo(&[&["pacman", "-S", "--needed"], packages].concat());
}

/// Copies `path` to `path.bak` once, so the original config is never lost.
pub fn backup(path: &str) {
    let bak = format!("{path}.bak");
    if !Path::new(&bak).exists() {
        ui::info(&format!("Backing up {path} to {bak}..."));
        sudo(&["cp", path, &bak]);
    }
}
