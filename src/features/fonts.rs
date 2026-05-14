use dialoguer::MultiSelect;
use crate::system::pacman::pacman_install;

pub fn fonts_installer() {
    let fonts = vec![
        "JetBrains Mono",
        "Fira Code",
        "Hack",
        "Iosevka",
        "Cascadia Code",
    ];

    let selected = MultiSelect::new()
        .with_prompt("Select fonts to install")
        .items(&fonts)
        .interact()
        .unwrap();

    if selected.is_empty() {
        println!("Nothing selected, skipping...");
        return;
    }

    for idx in &selected {
        match idx {
            0 => {
                println!("==> Installing JetBrains Mono...");
                pacman_install(&["ttf-jetbrains-mono-nerd"]);
            }
            1 => {
                println!("==> Installing Fira Code...");
                pacman_install(&["ttf-firacode-nerd"]);
            }
            2 => {
                println!("==> Installing Hack...");
                pacman_install(&["ttf-hack-nerd"]);
            }
            3 => {
                println!("==> Installing Iosevka...");
                pacman_install(&["ttf-iosevka-nerd"]);
            }
            4 => {
                println!("==> Installing Cascadia Code...");
                pacman_install(&["ttf-cascadia-code-nerd"]);
            }
            _ => {}
        }
    }

    println!("\n✓ Done! Fonts installed successfully.");
}