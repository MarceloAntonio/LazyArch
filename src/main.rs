use is_root::is_root;

mod arg;
mod features;
mod menu;
mod system;
mod ui;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if arg::parse_args(&args, VERSION) {
        return;
    }

    if !system::is_arch_based() || is_root() {
        ui::error("The system being used is not supported, or try the script without sudo or root");
        return;
    }

    ui::banner();
    menu::main_menu();
}