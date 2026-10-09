use crate::system::pacman_install;
use crate::ui;

const APPS: [(&str, &[&str]); 12] = [
    ("Firefox",      &["firefox"]),
    ("Chromium",     &["chromium"]),
    ("Thunderbird",  &["thunderbird"]),
    ("VLC",          &["vlc"]),
    ("mpv",          &["mpv"]),
    ("LibreOffice",  &["libreoffice-fresh"]),
    ("GIMP",         &["gimp"]),
    ("OBS Studio",   &["obs-studio"]),
    ("Discord",      &["discord"]),
    ("Telegram",     &["telegram-desktop"]),
    ("qBittorrent",  &["qbittorrent"]),
    ("KeePassXC",    &["keepassxc"]),
];

pub fn run() {
    let packages = ui::pick_packages("Select apps to install", &APPS);
    if packages.is_empty() {
        return;
    }

    pacman_install(&packages);
    ui::success("Apps installed!");
}
