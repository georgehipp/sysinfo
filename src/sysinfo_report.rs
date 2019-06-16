use os_info;
use sysinfo;
use std::collections::HashMap;
extern crate assert_cli;

// Public Function to be called by main.rs
cfg_if! {
    if #[cfg(unix)] {
        pub fn call(arg: &str) {
            use sysinfo::SystemExt;
            let mut system = sysinfo::System::new();
            let info = os_info::get();

            system.refresh_all();
            
            match arg {
                "os" => print_out(os(info)),
                "d" | "disk" => disk(system),
                "mem" | "memory" => memory(system),
                "proc" | "processes" => processes(system),
                "cpu" => cpu(system),
                "comp" | "components" => components(system),
                "net" | "network" => network(system),
                _ => println!("{:?} Not a Valid Option", arg),
            }
        }
    } else {
        pub fn call(arg: &str) {
            use sysinfo::SystemExt;
            let mut system = sysinfo::System::new();
            let info = os_info::get();

            system.refresh_all();
            
            match arg {
                "os" => print_out(os(info)),
                "d" | "disk" => disk(system),
                "mem" | "memory" => memory(system),
                "proc" | "processes" => processes(system),
                "cpu" => cpu(system),
                "net" | "network" => network(system),
                _ => println!("{:?} Not a Valid Option", arg),
            }
        }      
    }
}

fn print_out(value: HashMap<String, String>) {
    println!("{:?}", value);
}

// OS Information
fn os(info: os_info::Info) -> HashMap<String, String> {
    let mut return_value: HashMap<String, String> = HashMap::new();
    return_value.insert("Type:".to_string(), info.os_type().to_string());
    return_value.insert("Version:".to_string(), info.version().to_string());
    return_value
}

// Disk Information
fn disk(system: sysinfo::System) {
    use sysinfo::{DiskExt, SystemExt};

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
    println!("Total Memory: {} kB", system.get_total_memory());
    println!("Used Memory : {} kB", system.get_used_memory());
    println!("Total Swap  : {} kB", system.get_total_swap());
    println!("Used Swap   : {} kB", system.get_used_swap());
    println!(" --------------------");

    // Network data:
    println!(
        "Network Input Data - {} B",
        system.get_network().get_income()
    );
    println!(
        "Network Output Data - {} B",
        system.get_network().get_outcome()
    );
    println!(" --------------------");
}

// Complete Process List
fn processes(system: sysinfo::System) {
    use sysinfo::{ProcessExt, ProcessorExt, SystemExt};

    // Every process info:
    for (pid, proc_) in system.get_process_list() {
        println!(
            "Process PID - {} : Name - {} => Status - {:?}",
            pid,
            proc_.name(),
            proc_.status()
        );
        println!("  cmd - {:?}", proc_.cmd());
        println!("  exe - {:?}", proc_.exe());
        println!("  parent - {:?}", proc_.parent());
        println!("  environ - {:?}", proc_.environ());
        println!("  cwd - {:?}", proc_.cwd());
        println!("  root - {:?}", proc_.root());
        println!("  memory - {:?}", proc_.memory());
        println!("  start_time - {:?}", proc_.start_time());
        println!("  cpu_usage - {:?}", proc_.cpu_usage());
        // Not Currently Functioning
        //println!("  uid - {:?}", proc_.uid());
        //println!("  gid - {:?}", proc_.gid());
        //println!("  tasks - {:?}", proc_.tasks());
    }

    // Processor Information
    for processor in system.get_processor_list() {
        println!("Name - {:?}", processor.get_name());
        println!("Usage - {:?}", processor.get_cpu_usage());
    }
} 

fn cpu(system: sysinfo::System) {
    use sysinfo::SystemExt;

    // Number of processors Minus one due to it counting the array as one
    println!("NB processors: {}", system.get_processor_list().len() - 1);
}

cfg_if! {
    if #[cfg(unix)] {
        // List of all Components
        fn components(system: sysinfo::System) {
            use sysinfo::SystemExt;

            // Linux Only Components:
            for component in system.get_components_list() {
                println!("Components: {:?}", component);
            }
        }
    }
}

// Network Info - Needs to be validated
fn network(system: sysinfo::System) {
    use sysinfo::{NetworkExt, SystemExt};

    println!(" Input Data: {} B", system.get_network().get_income());
    println!("Output Data: {} B", system.get_network().get_outcome());
}
