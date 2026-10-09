use std::fs;
use crate::system::{is_enabled, pacman_install, succeeds, sudo};
use crate::ui;

pub fn done() -> bool {
    is_enabled("libvirtd.service")
}

pub fn run() {
    let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    if !cpuinfo.contains(" vmx") && !cpuinfo.contains(" svm") {
        ui::warn("CPU virtualization (Intel VT-x / AMD-V) is off or unsupported. Enable it in your BIOS/UEFI first.");
        return;
    }

    ui::info("Installing QEMU/KVM + virt-manager (if pacman asks to replace iptables, say yes)...");
    pacman_install(&["qemu-desktop", "libvirt", "virt-manager", "dnsmasq", "iptables-nft", "edk2-ovmf", "swtpm"]);
    sudo(&["systemctl", "enable", "--now", "libvirtd.service"]);

    let user = std::env::var("USER").expect("USER not set");
    sudo(&["usermod", "-aG", "libvirt", &user]);

    // Default NAT network, so VMs get internet. net-start fails if it's already running, that's fine.
    sudo(&["virsh", "net-autostart", "default"]);
    succeeds("sudo", &["virsh", "net-start", "default"]);

    ui::success("Virtualization ready! Log out and back in, then open 'Virtual Machine Manager'.");
}
