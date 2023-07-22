use sysinfo::{CpuExt, DiskExt, RefreshKind, System, SystemExt};

#[derive(Debug)]
pub struct Resources {
    hostname: String,
    total_memory: u64,
    used_memory: u64,
    total_space: u64,
    available_space: u64,
    cpu_amount: usize,
    pub cpu_usage: f32,
    system_struct: System,
}

impl Resources {
    pub(crate) fn refresh(&mut self) {
        self.cpu_usage = calc_cpu_usage(&mut self.system_struct);
        self.system_struct.refresh_memory();
        self.used_memory = self.system_struct.used_memory();
        self.used_memory = disk_info(&mut self.system_struct).1;
    }
}

//Goes through each disk to retrieve the total and available disk space on the system
fn disk_info(sys: &mut System) -> (u64, u64){
    sys.refresh_disks();
    let mut total = 0;
    let mut available = 0;
    for disk in sys.disks() {
        total += disk.total_space();
        available += disk.available_space();
    }
    (total, available)
}

//Divides the usage of each CPU with # of cores to get the total usage
fn calc_cpu_usage(sys: &mut System) -> f32 {
    sys.refresh_cpu();
    let mut total : f32 = 0.0;
    for cpu in sys.cpus(){
        total += cpu.cpu_usage();
    }
    total / sys.cpus().len() as f32
}

//Creates a System struct excluding the information we won't be using as of now
fn get_system() -> System {
    let mut sys = System::new_with_specifics(
        RefreshKind::everything()
            .without_components()
            .without_networks()
            .without_processes()
            .without_users_list()
    );
    sys.refresh_all();
    sys
}

//Breaks down the system struct into one with the variables we are interested in
pub fn retrieve_host_information() -> Resources{
    let mut sys = get_system();
    sys.refresh_all();
    let disk_space = disk_info(&mut sys);

    Resources {
        hostname: sys.host_name().unwrap(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_space: disk_space.0,
        available_space: disk_space.1,
        cpu_amount: sys.cpus().len(),
        cpu_usage: calc_cpu_usage(&mut sys),
        system_struct: sys,
    }
}
