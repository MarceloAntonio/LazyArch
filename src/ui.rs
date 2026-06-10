use colored::Colorize;

const VERSION: &str = env!("CARGO_PKG_VERSION");

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
    println!("  {} {}\n", "v".dimmed(), VERSION.dimmed());
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
