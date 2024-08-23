use std::mem::replace;
fn main() {
    use sysinfo::{
        Components, Disks, Networks, System,
    };
    let mut sys = System::new_all();
    sys.refresh_all();
    println!("OS: {:?}", System::name());
    println!("Kernel version: {:?}", System::kernel_version());
    println!("Total memory: {} KB", sys.total_memory());
    println!("Used memory: {} KB", sys.used_memory());
    println!("Free memory: {} KB", sys.free_memory());
}
