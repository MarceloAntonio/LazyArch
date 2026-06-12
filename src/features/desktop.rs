use std::process::Command;
use dialoguer::{Confirm, Select};
use crate::system::pacman::pacman_install;
use crate::ui;

pub fn desktop_installer() {
    let desktops: Vec<(&str, Vec<&str>)> = vec![
        ("i3 (Tiling WM, X11)",          vec!["i3-wm", "i3status", "i3lock", "dmenu", "feh", "picom", "xorg-server", "xorg-xinit"]),
        ("Hyprland (Tiling WM, Wayland)", vec!["hyprland", "waybar", "rofi", "hyprpaper", "xdg-desktop-portal-hyprland", "qt5-wayland", "qt6-wayland"]),
        ("Sway (Tiling WM, Wayland)",    vec!["sway", "swaybg", "waybar", "xorg-xwayland", "polkit"]),
        ("bspwm (Tiling WM, X11)",       vec!["bspwm", "sxhkd", "feh", "picom", "dmenu", "xorg-server", "xorg-xinit"]),
        ("Awesome (Tiling WM, X11)",     vec!["awesome", "picom", "dmenu", "feh", "xorg-server", "xorg-xinit"]),
        ("GNOME",                         vec!["gnome", "gnome-tweaks", "gdm"]),
        ("KDE Plasma",                    vec!["plasma", "kde-applications", "sddm"]),
        ("XFCE",                          vec!["xfce4", "xfce4-goodies"]),
        ("Cinnamon",                      vec!["cinnamon", "nemo"]),
        ("MATE",                          vec!["mate", "mate-extra"]),
    ];

    let names: Vec<&str> = desktops.iter().map(|(name, _)| *name).collect();

    let selected = Select::new()
        .with_prompt("Select a Desktop Environment or Window Manager")
        .items(&names)
        .default(0)
        .interact()
        .unwrap();

    let (name, packages) = &desktops[selected];
    ui::info(&format!("Installing {}...", name));
    pacman_install(packages);

    match *name {
        "GNOME" => {
            Command::new("sudo")
                .args(["systemctl", "enable", "gdm"])
                .status()
                .expect("Failed to enable GDM");
            ui::success("GDM enabled.");
        }
        "KDE Plasma" => {
            Command::new("sudo")
                .args(["systemctl", "enable", "sddm"])
                .status()
                .expect("Failed to enable SDDM");
            ui::success("SDDM enabled.");
        }
        _ => {
            let install_dm = Confirm::new()
                .with_prompt("Install a display manager? (SDDM)")
                .default(false)
                .interact()
                .unwrap();

            if install_dm {
                pacman_install(&["sddm"]);
                Command::new("sudo")
                    .args(["systemctl", "enable", "sddm"])
                    .status()
                    .expect("Failed to enable SDDM");
                ui::success("SDDM enabled.");
            }
        }
    }

    ui::success(&format!("{} installed! Reboot to apply changes.", name));
}