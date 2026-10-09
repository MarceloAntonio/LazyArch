use colored::Colorize;
use dialoguer::MultiSelect;

use crate::system::is_installed;

pub fn banner() {
    let art = r#"
  _                       _             _     
 | |    __ _ _____   _   / \   _ __ ___| |__  
 | |   / _` |_  / | | | / _ \ | '__/ __| '_ \ 
 | |__| (_| |/ /| |_| |/ ___ \| | | (__| | | |
 |_____\__,_/___|\__, /_/   \_\_|  \___|_| |_|
                  |___/                         
"#;
    println!("{}", art.bold().cyan());
    println!("  {} {}\n", "v".dimmed(), crate::VERSION.dimmed());
}

pub fn success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

pub fn info(msg: &str) {
    println!("{} {}", "==>".blue().bold(), msg);
}

pub fn warn(msg: &str) {
    println!("{} {}", "⚠".yellow().bold(), msg);
}

pub fn error(msg: &str) {
    eprintln!("{} {}", "✗".red().bold(), msg);
}

/// Multi-select over (name, packages). Installed entries start checked and labeled,
/// unchecking one never removes it. Returns the packages of every checked entry.
pub fn pick_packages(prompt: &str, items: &[(&str, &[&'static str])]) -> Vec<&'static str> {
    let installed: Vec<bool> = items.iter().map(|(_, packages)| is_installed(packages)).collect();
    let names: Vec<String> = items
        .iter()
        .zip(&installed)
        .map(|((name, _), &inst)| if inst { format!("{name} (installed)") } else { name.to_string() })
        .collect();

    let selected = MultiSelect::new()
        .with_prompt(prompt)
        .items(&names)
        .defaults(&installed)
        .interact()
        .unwrap();

    if selected.is_empty() {
        println!("Nothing selected, skipping...");
    }

    selected.iter().flat_map(|&idx| items[idx].1.iter().copied()).collect()
}
