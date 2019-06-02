use sysinfo;
use os_info;

// Public Function to be called by main.rs 
pub fn call(arg: &str) {
    use sysinfo::{SystemExt};
    let mut system = sysinfo::System::new();
    
    // Update all information of our system struct:
    system.refresh_all();

    match arg {
        "os" => os(),
        "d" | "disk" => disk(system),
        "mem" | "memory" => memory(system),
        "proc" | "processes" => processes(system),
        "cpu" => cpu(system),
        "comp" | "components" => components(system),
        "net" | "network" => network(system),
        _ => println!("{:?} Not a Valid Option", arg)
    }
}

// OS Information
fn os() {
    let info = os_info::get();

    // OS Information:
    println!("Type: {}", info.os_type());
    println!("Version: {}", info.version());
}

// Disk Information
fn disk(system: sysinfo::System) {
    use sysinfo::{DiskExt, SystemExt};

    // Disk information:
    for disk in system.get_disks() {
        println!("Disk Name - {:?}", disk.get_name());
        println!("Disk Type - {:?}", disk.get_type());
        // Does Not Appear to work in Windows
        println!("Disk File System - {:?}", disk.get_file_system());
        println!("Disk Mount Point - {:?}", disk.get_mount_point());
        println!("Disk Total Space - {:?}", disk.get_total_space());
        println!("Disk Available Space - {:?}", disk.get_available_space());
        println!(" --------------------");
    }
} 


// System Memory Usage
fn memory(system: sysinfo::System) {
    use sysinfo::{NetworkExt, SystemExt};

    // RAM and SWAP information:
    println!("total memory: {} kB", system.get_total_memory());
    println!("used memory : {} kB", system.get_used_memory());
    println!("total swap  : {} kB", system.get_total_swap());
    println!("used swap   : {} kB", system.get_used_swap());
    println!(" --------------------");

    // Network data:
    println!("Network Input Data - {} B", system.get_network().get_income());
    println!("Network Output Data - {} B", system.get_network().get_outcome());
    println!(" --------------------");
}


fn processes(system: sysinfo::System) {
    use sysinfo::{ProcessExt, ProcessorExt, SystemExt};

    // Every process info:
    for (pid, proc_) in system.get_process_list() {
        println!("Process PID - {} : Name - {} => Status - {:?}", pid, proc_.name(), proc_.status());
        println!("  cmd - {:?}", proc_.cmd());
        println!("  exe - {:?}", proc_.exe());
        println!("  parent - {:?}", proc_.parent());
        println!("  environ - {:?}", proc_.environ());
        println!("  cwd - {:?}", proc_.cwd());
        println!("  root - {:?}", proc_.root());
        println!("  memory - {:?}", proc_.memory());
        println!("  start_time - {:?}", proc_.start_time());
        println!("  cpu_usage - {:?}", proc_.cpu_usage());
        // Does not work on Windows
        //if  info.os_type() != os_info::Type::Windows {
        //    println!("  uid - {:?}", proc_.uid());
        //    println!("  gid - {:?}", proc_.gid());
        //    println!("  tasks - {:?}", proc_.tasks());
        //}
    }
    
    // Processor Information 
    for processor in system.get_processor_list() {
        println!("Name - {:?}", processor.get_name());
        println!("Usage - {:?}", processor.get_cpu_usage());
    }
}

fn cpu(system: sysinfo::System) {
    use sysinfo::{SystemExt};

    // Number of processors Minus one due to it counting the array as one
    println!("NB processors: {}", system.get_processor_list().len()-1);
}

fn components(system: sysinfo::System) {
    use sysinfo::{SystemExt};

    // Linux Only Components:
    for component in system.get_components_list() {
        println!("{:?}", component);
    }
}

fn network(system: sysinfo::System) {
    use sysinfo::{NetworkExt, SystemExt};

    // Network data:
    println!("input data : {} B", system.get_network().get_income());
    println!("output data: {} B", system.get_network().get_outcome());
}
