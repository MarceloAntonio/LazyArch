use std::path::Path;
use crate::system::{backup, pacman_install, sudo, write_root};
use crate::ui;

const CONF: &str = "/etc/systemd/zram-generator.conf";

pub fn done() -> bool {
    Path::new(CONF).exists()
}

pub fn run() {
    pacman_install(&["zram-generator"]);

    ui::info("Configuring compressed swap in RAM (half your RAM, up to 8 GB)...");
    backup(CONF);
    write_root(CONF, "[zram0]\nzram-size = min(ram / 2, 8192)\ncompression-algorithm = zstd\n");

    sudo(&["systemctl", "daemon-reload"]);
    sudo(&["systemctl", "start", "systemd-zram-setup@zram0.service"]);

    ui::success("zram enabled! Check it with: zramctl");
}
