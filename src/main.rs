use is_root::is_root;
mod menu;
mod features;
mod system;

fn main() {
    
    if !system::is_arch_based() || is_root(){
        println!("The system being used is not supported. or try the script without sudo or root")
    }
    else {
       menu::main_menu();
    }
 
}