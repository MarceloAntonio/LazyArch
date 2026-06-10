use is_root::is_root;

mod features;
mod menu;
mod system;
mod ui;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "--version" | "-v" => {
                println!("lazy-arch {}", VERSION);
                return;
            }
            "--help" | "-h" => {
                println!("lazy-arch {} — Automate your Arch Linux setup", VERSION);
                println!();
                println!("Usage: lazy-arch [OPTIONS]");
                println!();
                println!("Options:");
                println!("  -v, --version    Show version");
                println!("  -h, --help       Show this help");
                return;
            }
            _ => {
                ui::error(&format!("Unknown option: {}", args[1]));
                println!("Run 'lazy-arch --help' for usage.");
                return;
            }
        }
    }

    if !system::is_arch_based() || is_root() {
        ui::error("The system being used is not supported, or try the script without sudo or root");
        return;
    }

    ui::banner();
    menu::main_menu();
}