use std::process::Command;
use crate::system::pacman::pacman_install;
use crate::system::is_systemd_running::is_systemd_running;
use crate::ui;

pub fn install_docker() {
    pacman_install(&["docker", "docker-compose", "docker-buildx"]);

    if is_systemd_running() {
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
    } else {
        ui::success("Docker installed!");
        ui::warn("Systemd not running. Run manually after reboot:");
        println!("  sudo systemctl enable --now docker");
        println!("  sudo usermod -aG docker $USER");
    }
}