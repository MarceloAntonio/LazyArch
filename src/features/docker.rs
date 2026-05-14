use std::process::Command;
use crate::system::pacman::pacman_install;
use crate::system::is_systemd_running::is_systemd_running;


pub fn install_docker() {
    pacman_install(&["docker", "docker-compose"]);

    if is_systemd_running() {
        Command::new("sudo")
            .args(["systemctl", "enable", "--now", "docker"])
            .status()
            .unwrap();

        let user = std::env::var("USER").unwrap();
        Command::new("sudo")
            .args(["usermod", "-aG", "docker", &user])
            .status()
            .unwrap();

        println!("  Docker installed! Log out and back in to use without sudo.");
    } else {
        println!("  Docker installed!");
        println!("   Systemd not running (are you in a container?).");
        println!("   Run manually after reboot:");
        println!("   sudo systemctl enable --now docker");
        println!("   sudo usermod -aG docker $USER");
    }
}