mod resources;

use crate::resources::{retrieve_host_information};

fn main() {
    println!("Sup fools");
    let mut resources = retrieve_host_information();
    println!("{:?}", resources);

    loop {
        println!("CPU USAGE: {}%", resources.cpu_usage);
        resources.refresh();
        std::thread::sleep(std::time::Duration::from_millis(1000));
    } 
}
