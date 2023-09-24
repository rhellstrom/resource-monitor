use sysinfo::{CpuExt, DiskExt, RefreshKind, System, SystemExt};
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
    pub cpu_usage: f32,
    pub cpu_load_per_core: Vec<f32>,
    pub disk_names: Vec<String>,
    pub disk_mount: Vec<String>,
    pub disk_available: Vec<u64>,
    pub disk_total: Vec<u64>,
    uptime: u64,
    os_version: String,

    #[serde(skip_serializing)]
    system_struct: System,
}

impl Resources {
    /// Creates an instance of System and returns a Resources struct with desired system information
    pub fn new() -> Self {
        let mut sys = get_system();
        sys.refresh_all();
        let disk_space = disk_total_usage(&mut sys);
        let os_name = sys.long_os_version().unwrap_or_else(|| String::from("Unknown"));

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
            disk_mount: get_disks_mount(&mut sys),
            disk_available: get_disks_available(&mut sys),
            disk_total: get_disks_total(&mut sys),
            uptime: sys.uptime(),
            os_version: os_name,
            system_struct: sys,

        }
    }

    /// Refreshes the CPU, memory and disk usage
    pub(crate) fn refresh(&mut self) {
        self.system_struct.refresh_cpu();
        self.system_struct.refresh_memory();
        self.system_struct.refresh_disks();
        self.system_struct.refresh_disks_list();

        self.cpu_usage = self.system_struct.global_cpu_info().cpu_usage();


        self.used_memory = self.system_struct.used_memory();
        self.available_space = disk_total_usage(&mut self.system_struct).1;
        self.used_swap = self.system_struct.used_swap();
        self.cpu_load_per_core = get_cpu_load_per_core(&self.system_struct);
        self.disk_available = get_disks_available(&mut self.system_struct)
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap()
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

fn get_disks_mount(sys: &mut System) -> Vec<String> {
    sys.refresh_disks_list();
    let mut names = vec![];
    for disk in sys.disks() {
        names.push(disk.mount_point().to_string_lossy().to_string()) //This needs to be done cleaner
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

/// Returns a System struct using System::new_with_specifics and fills it with information using refresh.all()
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

