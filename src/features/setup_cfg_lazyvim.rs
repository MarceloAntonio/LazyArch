use std::fs;
use std::path::PathBuf;

pub fn setup_cfg_lazyvim() {
    let home = std::env::var("HOME").expect("HOME not set");
    
    let plugins_dir = PathBuf::from(&home).join(".config/nvim/lua/plugins");
    fs::create_dir_all(&plugins_dir).expect("Failed to create plugins directory");

    let catppuccin_config = r#"return {
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

    let config_path = plugins_dir.join("catppuccin.lua");
    fs::write(&config_path, catppuccin_config).expect("Failed to write catppuccin.lua");

    println!("✓ Catppuccin config written to {:?}", config_path);
}