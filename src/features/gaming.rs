use dialoguer::MultiSelect;
use crate::system::{backup, detect_gpus, is_installed, pacman_install, sudo};
use crate::ui;

const PACMAN_CONF: &str = "/etc/pacman.conf";

const EXTRAS: [(&str, &[&str]); 4] = [
    ("Steam",                            &["steam"]),
    ("Wine + Winetricks + Lutris",       &["wine", "winetricks", "lutris"]),
    ("Gamemode (performance optimizer)", &["gamemode", "lib32-gamemode"]),
    ("MangoHud (FPS overlay)",           &["mangohud", "lib32-mangohud"]),
];

fn multilib_enabled() -> bool {
    std::fs::read_to_string(PACMAN_CONF).is_ok_and(|c| c.lines().any(|l| l == "[multilib]"))
}

/// Multilib on and at least one gaming extra installed.
pub fn done() -> bool {
    multilib_enabled() && EXTRAS.iter().any(|(_, packages)| is_installed(packages))
}

fn enable_multilib() {
    if multilib_enabled() {
        ui::success("Multilib already enabled, skipping...");
        return;
    }

    backup(PACMAN_CONF);
    ui::info("Enabling multilib...");
    // Uncomment [multilib] and the Include line right below it.
    sudo(&["sed", "-i", "-e", "s/^#\\[multilib\\]/[multilib]/", "-e", "/^\\[multilib\\]/{n;s/^#//}", PACMAN_CONF]);
    sudo(&["pacman", "-Syu"]);

    ui::success("Multilib enabled!");
}

/// Driver packages for every GPU found, so hybrid laptops get both.
fn gpu_drivers() -> Vec<&'static str> {
    ui::info("Detecting GPU...");
    let gpus = detect_gpus();

    let mut packages = vec![];
    if gpus.intel {
        println!("  🔵 Intel GPU detected");
        packages.extend(["mesa", "vulkan-intel", "lib32-mesa", "lib32-vulkan-intel"]);
    }
    if gpus.amd {
        println!("  🔴 AMD GPU detected");
        packages.extend(["xf86-video-amdgpu", "mesa", "vulkan-radeon", "lib32-mesa", "lib32-vulkan-radeon"]);
    }
    if gpus.nvidia {
        println!("  🟢 NVIDIA GPU detected");
        // ponytail: nvidia-open covers Turing (GTX 16xx/RTX 20xx) and newer on the stock kernel;
        // add nvidia-open-dkms for linux-lts/zen and AUR legacy drivers for older cards if users ask.
        packages.extend(["nvidia-open", "nvidia-utils", "nvidia-settings", "lib32-nvidia-utils"]);
    }

    if packages.is_empty() {
        ui::warn("Could not detect GPU. Install drivers manually.");
    }

    packages.sort_unstable();
    packages.dedup();
    packages
}

pub fn run() {
    println!("\n🎮 Gaming Setup\n");

    // Multilib first: the lib32-* driver packages live there.
    enable_multilib();
    let mut packages = gpu_drivers();

    let names: Vec<&str> = EXTRAS.iter().map(|(name, _)| *name).collect();

    let selected = MultiSelect::new()
        .with_prompt("Select what you want to install")
        .items(&names)
        .defaults(&[true; 4])
        .interact()
        .unwrap();

    for &idx in &selected {
        packages.extend(EXTRAS[idx].1);
    }

    if packages.is_empty() {
        println!("Nothing to install, skipping...");
        return;
    }

    pacman_install(&packages);
    ui::success("Gaming setup complete!");

    if selected.contains(&0) {
        println!("  Tip: Enable Proton in Steam > Settings > Compatibility.");
    }
}
