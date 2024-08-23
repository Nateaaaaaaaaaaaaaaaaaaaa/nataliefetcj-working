use std::mem::replace;
<<<<<<< HEAD

=======
>>>>>>> 8653b6773ae0ff4eda970d65807d1b2d48d6abee
fn main() {
    use sysinfo::{
        Components, Disks, Networks, System,
    };
    let mut sys = System::new_all();
    sys.refresh_all();
<<<<<<< HEAD
    println!(" ⣿⣿⣿⠿⠿⣿⣿⡿⢋⣶⣶⣬⣙⠿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿  OS: {:?}", System::name());
    println!(" ⣿⡿⢡⣿⣷⣶⣦⣥⣿⣿⣿⣿⣿⣷⣮⡛⢿⣿⣿⣿⣿⣿⣿⣿  Kernel version: {:?}", System::kernel_version());
    println!(" ⣿⡇⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠿⢮⡙⣿⣿⣯⢐⡎⣿  Total memory: {} KB", sys.total_memory());
    println!(" ⣿⢹⣿⣿⣿⣿⣿⣿⡿⣡⡬⢿⣿⣿⣿⣶⣶⣼⣦⠥⣖⣩⣾⣿  Used memory: {} KB", sys.used_memory());
    println!(" ⣿⢸⣿⣿⣿⡿⣿⣿⣿⣿⠇⣌⢛⣻⣿⣿⣟⣛⣿⣧⠹⣿⣿⣿  Free memory: {} KB", sys.free_memory());
    println!(" ⠏⣼⣿⣿⢏⣾⣿⣟⣩⣶⣶⣿⣿⣿⣿⣿⡟⡿⢸⡿⣡⣿⣿⣿  Total swap: {} KB", sys.total_swap());
    println!(" ⣼⣿⣿⠇⣼⣿⣿⢸⠋⠁⠉⢽⣿⣿⣿⣟⣠⣤⣆⢃⢻⣿⣿⣿");
    println!(" ⣿⣿⣿⣼⣿⣿⣿⡞⣿⣿⣷⣾⣿⣿⣿⣿⡿⠟⠛⠸⢦⣙⡋⣿");
    println!(" ⣿⣿⣿⠹⣿⣿⡿⠗⣈⣭⣭⣭⣉⠻⡟⣩⣶⣾⣿⣿⣶⡙⣱⣿");
    println!(" ⣿⣿⣿⣷⣌⡛⠠⣿⣿⣿⣿⣿⣿⣿⣾⣿⣿⣿⣿⣿⣿⣿⢸⣿");
    println!(" ⣿⣿⣿⣿⢏⣴⣧⣴⡘⣿⣿⣿⣿⣿⣿⣿⣿⣿⣱⣶⣴⡜⢸⣿");
    println!(" ⣿⣿⣿⢃⣾⣿⣿⣿⡷⠉⢿⣿⣿⣿⣿⣿⣿⢰⣾⣿⣿⣧⢸⣿ ");
}
=======
    println!("OS: {:?}", System::name());
    println!("Kernel version: {:?}", System::kernel_version());
    println!("Total memory: {} KB", sys.total_memory());
    println!("Used memory: {} KB", sys.used_memory());
    println!("Free memory: {} KB", sys.free_memory());
}
>>>>>>> 8653b6773ae0ff4eda970d65807d1b2d48d6abee
