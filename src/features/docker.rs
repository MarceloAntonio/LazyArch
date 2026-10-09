use crate::system::{is_enabled, is_installed, pacman_install, sudo};
use crate::ui;

pub fn done() -> bool {
    is_installed(&["docker"]) && is_enabled("docker")
}

pub fn run() {
    pacman_install(&["docker", "docker-compose", "docker-buildx"]);
    sudo(&["systemctl", "enable", "--now", "docker"]);

    let user = std::env::var("USER").expect("USER not set");
    sudo(&["usermod", "-aG", "docker", &user]);

    ui::success("Docker installed! Log out and back in to use without sudo.");
}
