use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{exit, Command, ExitStatus, Stdio};

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
    check(cmd, args, Command::new(cmd).args(args).status());
}

fn check(cmd: &str, args: &[&str], result: io::Result<ExitStatus>) {
    let error = match result {
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

/// Writes a root-owned file (under /etc) by piping `content` into `sudo tee`.
pub fn write_root(path: &str, content: &str) {
    let result = Command::new("sudo")
        .args(["tee", path])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .and_then(|mut child| {
            child.stdin.take().expect("stdin is piped").write_all(content.as_bytes())?;
            child.wait()
        });
    check("sudo", &["tee", path], result);
}

pub fn sudo(args: &[&str]) {
    run("sudo", args);
}

/// Runs a command silently and reports whether it succeeded. For checks only.
pub fn succeeds(cmd: &str, args: &[&str]) -> bool {
    Command::new(cmd)
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// True only if every package is installed.
pub fn is_installed(packages: &[&str]) -> bool {
    succeeds("pacman", &[&["-Q"], packages].concat())
}

pub fn is_enabled(unit: &str) -> bool {
    succeeds("systemctl", &["is-enabled", "--quiet", unit])
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
