use std::process::Command;

pub fn pacman_cfg(){

        let pacman_path = "/etc/pacman.conf";

        println!("==> Enabling Colors");
        Command::new("sudo")
        .args(["sed", "-i", "s/#Color/Color/", pacman_path])
        .status()
        .expect("Failed to write the pacman.conf file");

        println!("==> Enabling parallel downloads");
        Command::new("sudo")
        .args(["sed", "-i", "s/#ParallelDownloads = 5/ParallelDownloads = 5/", pacman_path])
        .status()
        .expect("Failed to write the pacman.conf file");
    
        println!("==> Enabling progress bar");
        Command::new("sudo")
        .args(["sed", "-i", "s/^NoProgressBar/#NoProgressBar/", pacman_path])
        .status()
        .expect("Failed to write the pacman.conf file");


        println!("==> Adding ILoveCandy");
        Command::new("sudo")
        .args(["sed", "-i", "/^Color$/a ILoveCandy", pacman_path])
        .status()
        .expect("Failed to write the pacman.conf file");

}