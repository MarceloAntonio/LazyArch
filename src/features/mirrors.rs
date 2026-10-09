use dialoguer::Confirm;
use crate::system::{backup, is_enabled, pacman_install, sudo, write_root};
use crate::ui;

const MIRRORLIST: &str = "/etc/pacman.d/mirrorlist";
const REFLECTOR_CONF: &str = "/etc/xdg/reflector/reflector.conf";
const REFLECTOR_ARGS: [&str; 6] = ["--latest", "20", "--protocol", "https", "--sort", "rate"];

/// reflector writes its name in the mirrorlist header.
pub fn done() -> bool {
    std::fs::read_to_string(MIRRORLIST).is_ok_and(|c| c.to_lowercase().contains("reflector"))
}

pub fn run() {
    pacman_install(&["reflector"]);

    ui::info("Finding the fastest up-to-date mirrors...");
    sudo(&[&["reflector"], REFLECTOR_ARGS.as_slice(), &["--save", MIRRORLIST]].concat());

    let auto = Confirm::new()
        .with_prompt("Refresh mirrors automatically every week?")
        .default(!is_enabled("reflector.timer"))
        .interact()
        .unwrap();

    if auto {
        // reflector.timer reads its arguments from this file, one option per line.
        let conf: String = REFLECTOR_ARGS
            .chunks(2)
            .map(|pair| pair.join(" ") + "\n")
            .chain([format!("--save {MIRRORLIST}\n")])
            .collect();
        backup(REFLECTOR_CONF);
        write_root(REFLECTOR_CONF, &conf);
        sudo(&["systemctl", "enable", "reflector.timer"]);
    }

    // -Syu, never -Sy: a sync without upgrade is an unsupported partial upgrade.
    sudo(&["pacman", "-Syu"]);

    ui::success("Mirrors updated!");
}
