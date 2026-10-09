use std::process::Command;
use dialoguer::MultiSelect;
use crate::system::{self, is_enabled, pacman_install, sudo};
use crate::ui;

fn remove_orphans() {
    ui::info("Checking for orphaned packages...");

    let output = Command::new("pacman")
        .args(["-Qtdq"])
        .output()
        .expect("Failed to run pacman -Qtdq");

    let orphans = String::from_utf8_lossy(&output.stdout);
    let orphan_list: Vec<&str> = orphans.lines().collect();

    if orphan_list.is_empty() {
        ui::success("No orphaned packages found!");
        return;
    }

    ui::info(&format!("Found {} orphan(s): {}", orphan_list.len(), orphan_list.join(", ")));

    sudo(&[&["pacman", "-Rns"], orphan_list.as_slice()].concat());

    ui::success("Orphaned packages removed!");
}

fn clean_pacman_cache() {
    pacman_install(&["pacman-contrib"]);

    ui::info("Cleaning package cache (keeping last 3 versions)...");
    sudo(&["paccache", "-r"]);

    ui::info("Removing cache of uninstalled packages...");
    sudo(&["paccache", "-ruk0"]);

    ui::success("Pacman cache cleaned!");
}

fn enable_auto_cache_cleanup() {
    pacman_install(&["pacman-contrib"]);
    ui::info("Enabling weekly cache cleanup (keeps the last 3 versions)...");
    sudo(&["systemctl", "enable", "--now", "paccache.timer"]);
    ui::success("Pacman cache will be cleaned automatically every week!");
}

fn clean_journal_logs() {
    ui::info("Current journal disk usage:");
    system::run("journalctl", &["--disk-usage"]);

    ui::info("Cleaning logs older than 2 weeks...");
    sudo(&["journalctl", "--vacuum-time=2weeks"]);

    ui::success("Journal logs cleaned!");
}

fn check_failed_services() {
    ui::info("Checking for failed systemd services...\n");
    system::run("systemctl", &["--failed"]);

    println!();
    ui::info("Recent critical errors (current boot):\n");
    system::run("journalctl", &["-p", "3", "-xb", "--no-pager"]);
}

pub fn run() {
    let options = vec![
        "Remove Orphaned Packages",
        "Clean Pacman Cache",
        "Clean Systemd Journal Logs",
        "Check Failed Services & Errors",
        "Clean Pacman Cache Weekly (automatic)",
    ];

    let selected = MultiSelect::new()
        .with_prompt("Select maintenance tasks to run")
        .items(&options)
        .defaults(&[true, true, true, true, !is_enabled("paccache.timer")])
        .interact()
        .unwrap();

    if selected.is_empty() {
        println!("Nothing selected, skipping...");
        return;
    }

    for idx in &selected {
        match idx {
            0 => remove_orphans(),
            1 => clean_pacman_cache(),
            2 => clean_journal_logs(),
            3 => check_failed_services(),
            4 => enable_auto_cache_cleanup(),
            _ => {}
        }
    }

    ui::success("Maintenance complete!");
}
