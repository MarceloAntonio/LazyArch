use std::process::Command;
use crate::system::pacman_install;
use crate::ui;

pub fn run() {
    pacman_install(&["docker", "docker-compose", "docker-buildx"]);

    Command::new("sudo")
        .args(["systemctl", "enable", "--now", "docker"])
        .status()
        .expect("Failed to enable docker service");

    let user = std::env::var("USER").expect("USER not set");
    Command::new("sudo")
        .args(["usermod", "-aG", "docker", &user])
        .status()
        .expect("Failed to add user to docker group");

    ui::success("Docker installed! Log out and back in to use without sudo.");
}