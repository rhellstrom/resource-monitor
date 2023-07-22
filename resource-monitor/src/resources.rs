use sysinfo::{CpuExt, DiskExt, RefreshKind, System, SystemExt};
use serde::{Serialize};


#[derive(Serialize, Debug)]
pub struct Resources {
    hostname: String,
    total_memory: u64,
    used_memory: u64,
    total_space: u64,
    available_space: u64,
    cpu_amount: usize,
    pub cpu_usage: f32,
    #[serde(skip_serializing)]
    system_struct: System,
}

impl Resources {
    pub fn new() -> Self {
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
            cpu_usage: sys.global_cpu_info().cpu_usage(),
            system_struct: sys,
        }

    }

    pub(crate) fn refresh(&mut self) {
        self.system_struct.refresh_cpu();
        self.cpu_usage = self.system_struct.global_cpu_info().cpu_usage();
        self.system_struct.refresh_memory();
        self.used_memory = self.system_struct.used_memory();
        self.used_memory = disk_info(&mut self.system_struct).1;
    }
    //TODO: method for serializing to JSON
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

//Creates a System struct excluding the information we won't be using as of now
fn get_system() -> System {
    let mut sys = System::new_with_specifics(
        RefreshKind::everything()
            .without_components()
            .without_networks()
            .without_processes()
            .without_users_list()
            .without_networks_list()
    );
    sys.refresh_all();
    sys
}

