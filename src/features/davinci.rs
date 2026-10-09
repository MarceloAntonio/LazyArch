use dialoguer::{Confirm, Select};
use crate::features::aur;
use crate::system::{self, detect_gpus, is_installed, pacman_install};
use crate::ui;

const EDITIONS: [(&str, &str); 2] = [
    ("Free", "davinci-resolve"),
    ("Studio (needs a paid license)", "davinci-resolve-studio"),
];

pub fn done() -> bool {
    EDITIONS.iter().any(|(_, package)| is_installed(&[package]))
}

/// The one feature that uses the AUR: the `davinci-resolve` package is maintained
/// there and handles Blackmagic's download, so we never run remote scripts.
pub fn run() {
    let Some(helper) = aur::installed_helper() else {
        ui::warn("DaVinci Resolve comes from the AUR. Install an AUR helper first (Base System > Install AUR Helper).");
        return;
    };

    let names: Vec<&str> = EDITIONS.iter().map(|(name, _)| *name).collect();
    let idx = Select::new()
        .with_prompt("Select a DaVinci Resolve edition")
        .items(&names)
        .default(0)
        .interact()
        .unwrap();

    let go = Confirm::new()
        .with_prompt("It downloads ~3 GB and needs ~12 GB free while building (more for Studio). Continue?")
        .default(true)
        .interact()
        .unwrap();
    if !go {
        return;
    }

    // Resolve needs GPU compute (OpenCL/CUDA) to run at all.
    let gpus = detect_gpus();
    let mut compute = vec![];
    if gpus.nvidia {
        compute.push("opencl-nvidia");
    }
    if gpus.amd {
        compute.push("rocm-opencl-runtime");
    }
    if gpus.intel {
        compute.push("intel-compute-runtime");
    }
    if !compute.is_empty() {
        ui::info("Installing GPU compute support...");
        pacman_install(&compute);
    }

    ui::info(&format!("Building {} with {helper} (this takes a while)...", EDITIONS[idx].1));
    system::run(helper, &["-S", EDITIONS[idx].1]);

    ui::success("DaVinci Resolve installed! Open it from your app menu.");
}
