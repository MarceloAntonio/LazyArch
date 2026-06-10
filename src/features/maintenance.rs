use std::process::Command;
use dialoguer::MultiSelect;
use crate::system::pacman::pacman_install;
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

    Command::new("sudo")
        .args(["pacman", "-Rns", "--noconfirm"])
        .args(&orphan_list)
        .status()
        .expect("Failed to remove orphaned packages");

    ui::success("Orphaned packages removed!");
}

fn clean_pacman_cache() {
    pacman_install(&["pacman-contrib"]);

    ui::info("Cleaning package cache (keeping last 3 versions)...");
    Command::new("sudo")
        .args(["paccache", "-r"])
        .status()
        .expect("Failed to run paccache");

    ui::info("Removing cache of uninstalled packages...");
    Command::new("sudo")
        .args(["paccache", "-ruk0"])
        .status()
        .expect("Failed to run paccache");

    ui::success("Pacman cache cleaned!");
}

fn clean_journal_logs() {
    ui::info("Current journal disk usage:");
    Command::new("journalctl")
        .args(["--disk-usage"])
        .status()
        .expect("Failed to check journal size");

    ui::info("Cleaning logs older than 2 weeks...");
    Command::new("sudo")
        .args(["journalctl", "--vacuum-time=2weeks"])
        .status()
        .expect("Failed to clean journal logs");

    ui::success("Journal logs cleaned!");
}

fn check_failed_services() {
    ui::info("Checking for failed systemd services...\n");
    Command::new("systemctl")
        .args(["--failed"])
        .status()
        .expect("Failed to check services");

    println!();
    ui::info("Recent critical errors (current boot):\n");
    Command::new("journalctl")
        .args(["-p", "3", "-xb", "--no-pager"])
        .status()
        .expect("Failed to read journal errors");
}

pub fn maintenance_menu() {
    let options = vec![
        "Remove Orphaned Packages",
        "Clean Pacman Cache",
        "Clean Systemd Journal Logs",
        "Check Failed Services & Errors",
    ];

    let selected = MultiSelect::new()
        .with_prompt("Select maintenance tasks to run")
        .items(&options)
        .defaults(&[true, true, true, true])
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
            _ => {}
        }
    }

    ui::success("Maintenance complete!");
}
