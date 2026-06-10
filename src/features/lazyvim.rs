use std::fs;
use std::path::PathBuf;
use std::process::Command;
use dialoguer::Confirm;
use crate::system::pacman::pacman_install;
use crate::ui;

fn setup_catppuccin() {
    let home = std::env::var("HOME").expect("HOME not set");
    let plugins_dir = PathBuf::from(&home).join(".config/nvim/lua/plugins");
    fs::create_dir_all(&plugins_dir).expect("Failed to create plugins directory");

    let config = r#"return {
  {
    'catppuccin/nvim',
    name = 'catppuccin',
    priority = 1000,
    opts = {
      flavour = 'mocha',
      transparent_background = true,
      integrations = {
        telescope = true,
        mason = true,
        neotree = true,
        which_key = true,
        navic = { enabled = true },
        mini = true,
      },
    },
  },
  {
    'LazyVim/LazyVim',
    opts = {
      colorscheme = 'catppuccin-mocha',
    },
  },
}
"#;

    let path = plugins_dir.join("catppuccin.lua");
    fs::write(&path, config).expect("Failed to write catppuccin.lua");
    ui::success(&format!("Catppuccin config written to {:?}", path));
}

pub fn install_lazy_vim() {
    let home = std::env::var("HOME").expect("HOME not set");
    let nvim_path = format!("{}/.config/nvim", home);
    let nvim_git = format!("{}/.config/nvim/.git", home);

    let customization = Confirm::new()
        .with_prompt("Add Catppuccin theme customization?")
        .default(true)
        .interact()
        .unwrap();

    ui::info("Installing dependencies...");
    pacman_install(&["neovim", "git"]);

    ui::info("Cloning LazyVim starter...");
    Command::new("git")
        .args(["clone", "https://github.com/LazyVim/starter", &nvim_path])
        .status()
        .expect("Failed to clone LazyVim");

    if customization {
        ui::info("Applying Catppuccin theme...");
        setup_catppuccin();
    }

    Command::new("nvim")
        .args(["--headless", "+Lazy!", "sync", "+qa"])
        .status()
        .expect("Failed to sync LazyVim plugins");

    Command::new("rm")
        .args(["-rf", &nvim_git])
        .status()
        .expect("Failed to clean .git directory");

    ui::success("LazyVim installed!");
}