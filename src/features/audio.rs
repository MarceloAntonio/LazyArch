use crate::system::{is_installed, pacman_install};
use crate::ui;

const PACKAGES: [&str; 5] = ["pipewire", "wireplumber", "pipewire-pulse", "pipewire-alsa", "pipewire-jack"];

pub fn done() -> bool {
    is_installed(&PACKAGES)
}

pub fn run() {
    ui::info("Installing PipeWire (if pacman asks to replace PulseAudio or JACK, say yes)...");
    pacman_install(&PACKAGES);
    ui::success("Audio ready! Log out and back in to start PipeWire.");
}
