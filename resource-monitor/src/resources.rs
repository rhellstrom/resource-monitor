use sysinfo::{CpuExt, DiskExt, NetworkExt, RefreshKind, System, SystemExt};
use serde::{Serialize};


#[derive(Serialize, Debug)]
pub struct Resources {
    hostname: String,
    total_memory: u64,
    used_memory: u64,
    total_space: u64,
    available_space: u64,
    used_swap: u64,
    total_swap: u64,
    cpu_amount: usize,
    cpu_usage: f32,
    cpu_load_per_core: Vec<f32>,
    disk_names: Vec<String>,
    disk_available: Vec<u64>,
    disk_total: Vec<u64>,
    uptime: u64,
    os_version: String,
    kernel_version: String,
    load_avg_one: f64,
    load_avg_five: f64,
    load_avg_fifteen: f64,
    bytes_received: u64,
    bytes_transmitted: u64,

    #[serde(skip_serializing)]
    system_struct: System,
}

impl Resources {
    /// Creates an instance of System and returns a Resources struct with desired system information
    pub fn new() -> Self {
        let mut sys = get_system();
        sys.refresh_all();
        let disk_space = disk_total_usage(&mut sys);
        let os_version = sys.long_os_version().unwrap_or_else(|| String::from("Unknown"));
        let kernel_version = sys.kernel_version().unwrap_or_else(|| String::from("Unknown"));

        Resources {
            hostname: sys.host_name().unwrap(),
            total_memory: sys.total_memory(),
            used_memory: sys.used_memory(),
            total_space: disk_space.0,
            available_space: disk_space.1,
            used_swap: sys.used_swap(),
            total_swap: sys.total_swap(),
            cpu_amount: sys.cpus().len(),
            cpu_usage: sys.global_cpu_info().cpu_usage(),
            cpu_load_per_core: get_cpu_load_per_core(&sys),
            disk_names: get_disks_names(&mut sys),
            disk_available: get_disks_available(&mut sys),
            disk_total: get_disks_total(&mut sys),
            uptime: sys.uptime(),
            os_version,
            kernel_version,
            load_avg_one: sys.load_average().one,
            load_avg_five: sys.load_average().five,
            load_avg_fifteen: sys.load_average().fifteen,
            bytes_received: get_total_received(&sys),
            bytes_transmitted: get_total_transmitted(&sys),
            system_struct: sys,

        }
    }

    /// Refreshes the desired fields of our Resources struct
    pub(crate) fn refresh(&mut self) {
        self.system_struct.refresh_all();
        self.cpu_usage = self.system_struct.global_cpu_info().cpu_usage();
        self.used_memory = self.system_struct.used_memory();
        self.available_space = disk_total_usage(&mut self.system_struct).1;
        self.used_swap = self.system_struct.used_swap();
        self.cpu_load_per_core = get_cpu_load_per_core(&self.system_struct);
        self.disk_available = get_disks_available(&mut self.system_struct);
        self.uptime = self.system_struct.uptime();
        self.load_avg_one = self.system_struct.load_average().one;
        self.load_avg_five = self.system_struct.load_average().five;
        self.load_avg_fifteen = self.system_struct.load_average().fifteen;
        self.bytes_transmitted = get_total_transmitted(&self.system_struct);
        self.bytes_received = get_total_received(&self.system_struct);
    }

    pub fn serialize(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// Iterates through each disk summing up the total and available space on the system
fn disk_total_usage(sys: &mut System) -> (u64, u64){
    sys.refresh_disks();
    let mut total = 0;
    let mut available = 0;
    for disk in sys.disks() {
        total += disk.total_space();
        available += disk.available_space();
    }
    (total, available)
}

fn get_disks_names(sys: &mut System) -> Vec<String> {
    let mut names = vec![];
    for disk in sys.disks() {
        names.push(disk.name().to_string_lossy().to_string()) //This needs to be done cleaner
    }
    names
}

fn get_disks_available(sys: &mut System) -> Vec<u64> {
    sys.refresh_disks_list();
    let mut names = vec![];
    for disk in sys.disks() {
        names.push(disk.available_space()) //This needs to be done cleaner
    }
    names
}

fn get_disks_total(sys: &mut System) -> Vec<u64> {
    sys.refresh_disks_list();
    let mut names = vec![];
    for disk in sys.disks() {
        names.push(disk.total_space()) //This needs to be done cleaner
    }
    names
}

fn get_cpu_load_per_core(sys: &System) -> Vec<f32> {
    let mut load_per_core = Vec::new();
    for cpu in sys.cpus() {
        load_per_core.push(cpu.cpu_usage());
    }
    load_per_core
}

///Iterates over interfaces and returns total bytes received
fn get_total_received(sys: &System) -> u64 {
    let networks = sys.networks();
    let mut total = 0;
    for (_, network) in networks {
        total += network.total_received();
    }
    total
}

///Iterates over interfaces and returns total bytes transmitted
fn get_total_transmitted(sys: &System) -> u64 {
    let networks = sys.networks();
    let mut total = 0;
    for (_, network) in networks {
        total += network.total_transmitted();
    }
    total
}

/// Returns a System struct using System::new_with_specifics and fills it with information using refresh.all()
fn get_system() -> System {
    let mut sys = System::new_with_specifics(
        RefreshKind::everything()
            .without_components()
            .without_processes()
            .without_users_list()
    );
    sys.refresh_all();
    sys
}
