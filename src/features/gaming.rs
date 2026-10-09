use std::process::Command;
use dialoguer::MultiSelect;
use crate::system::pacman::pacman_install;
use crate::ui;

enum GpuVendor {
    Intel,
    Amd,
    Nvidia,
    Unknown,
}

fn detect_gpu() -> GpuVendor {
    let output = Command::new("lspci")
        .output()
        .expect("Failed to run lspci");

    let text = String::from_utf8_lossy(&output.stdout).to_lowercase();

    for line in text.lines() {
        if line.contains("vga") || line.contains("3d controller") || line.contains("display") {
            if line.contains("nvidia") { return GpuVendor::Nvidia; }
            if line.contains("amd") || line.contains("radeon") || line.contains("advanced micro") { return GpuVendor::Amd; }
            if line.contains("intel") { return GpuVendor::Intel; }
        }
    }

    GpuVendor::Unknown
}

fn enable_multilib() {
    let content = std::fs::read_to_string("/etc/pacman.conf")
        .expect("Failed to read pacman.conf");

    if !content.contains("#[multilib]") {
        ui::success("Multilib already enabled, skipping...");
        return;
    }

    ui::info("Enabling multilib...");

    Command::new("sudo")
        .args(["sed", "-i", "s/^#\\[multilib\\]/[multilib]/", "/etc/pacman.conf"])
        .status()
        .expect("Failed to uncomment [multilib]");

    Command::new("sudo")
        .args(["sed", "-i", "/^\\[multilib\\]/{n;s/^#//}", "/etc/pacman.conf"])
        .status()
        .expect("Failed to uncomment multilib Include");

    Command::new("sudo")
        .args(["pacman", "-Sy"])
        .status()
        .expect("Failed to sync pacman");

    ui::success("Multilib enabled!");
}

fn install_gpu_drivers() {
    ui::info("Detecting GPU...");

    match detect_gpu() {
        GpuVendor::Intel => {
            println!("  🔵 Intel GPU detected");
            pacman_install(&["xf86-video-intel", "mesa", "vulkan-intel", "lib32-mesa", "lib32-vulkan-intel"]);
        }
        GpuVendor::Amd => {
            println!("  🔴 AMD GPU detected");
            pacman_install(&["xf86-video-amdgpu", "mesa", "vulkan-radeon", "lib32-mesa", "lib32-vulkan-radeon"]);
        }
        GpuVendor::Nvidia => {
            println!("  🟢 NVIDIA GPU detected");
            pacman_install(&["nvidia", "nvidia-utils", "nvidia-settings", "lib32-nvidia-utils"]);
        }
        GpuVendor::Unknown => {
            ui::warn("Could not detect GPU. Install drivers manually.");
            return;
        }
    }

    ui::success("GPU drivers installed!");
}

pub fn gaming_setup() {
    println!("\n🎮 Gaming Setup\n");

    install_gpu_drivers();
    enable_multilib();

    let options = vec![
        "Steam",
        "Wine + Winetricks + Lutris",
        "Gamemode (performance optimizer)",
        "MangoHud (FPS overlay)",
    ];

    let selected = MultiSelect::new()
        .with_prompt("Select what you want to install")
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
            0 => {
                ui::info("Installing Steam...");
                pacman_install(&["steam"]);
            }
            1 => {
                ui::info("Installing Wine + Lutris...");
                pacman_install(&["wine", "winetricks", "lutris"]);
            }
            2 => {
                ui::info("Installing Gamemode...");
                pacman_install(&["gamemode", "lib32-gamemode"]);
            }
            3 => {
                ui::info("Installing MangoHud...");
                pacman_install(&["mangohud", "lib32-mangohud"]);
            }
            _ => {}
        }
    }

    ui::success("Gaming setup complete!");

    if selected.contains(&0) {
        println!("  Tip: Enable Proton in Steam > Settings > Compatibility.");
    }
}