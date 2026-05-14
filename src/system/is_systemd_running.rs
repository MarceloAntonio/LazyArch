use std::process::Command;

pub fn is_systemd_running() -> bool {
    Command::new("systemctl")
        .arg("is-system-running")
        .output()
        .map(|o| {
            let out = String::from_utf8_lossy(&o.stdout);
            !out.trim().contains("offline") && !out.trim().contains("unknown")
        })
        .unwrap_or(false)
}