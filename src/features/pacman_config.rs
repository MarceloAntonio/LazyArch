use crate::system::{backup, sudo};
use crate::ui;

const PATH: &str = "/etc/pacman.conf";

pub fn run() {
    backup(PATH);

    ui::info("Enabling colors, parallel downloads and progress bar...");
    sudo(&[
        "sed", "-i",
        "-e", "s/^#Color/Color/",
        "-e", "s/^#ParallelDownloads/ParallelDownloads/",
        "-e", "s/^NoProgressBar/#NoProgressBar/",
        PATH,
    ]);

    let content = std::fs::read_to_string(PATH).expect("Failed to read pacman.conf");
    if !content.contains("ILoveCandy") {
        ui::info("Adding ILoveCandy...");
        sudo(&["sed", "-i", "/^Color$/a ILoveCandy", PATH]);
    }

    ui::success("Pacman configured!");
}
