use os_info;
use std::collections::HashMap;
use sysinfo;

pub fn print_out(value: HashMap<String, String>, mut writer: impl std::io::Write) {
    match writeln!(writer, "{:?}", value) {
        Ok(_) => (),
        Err(error) => {
            panic!("Issue with std::io::write. {:?}", error)
        } 
    }
    println!("{:?}", value);
}

// OS Information
pub fn os(info: os_info::Info) -> HashMap<String, String> {
    let mut return_value: HashMap<String, String> = HashMap::new();
    return_value.insert("Type:".to_string(), info.os_type().to_string());
    return_value.insert("Version:".to_string(), info.version().to_string());
    return_value
}

// Disk Information
pub fn disk(system: sysinfo::System) {
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
pub fn memory(system: sysinfo::System) {
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
pub fn processes(system: sysinfo::System) {
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

pub fn cpu(system: sysinfo::System) {
    use sysinfo::SystemExt;

    // Number of processors Minus one due to it counting the array as one
    println!("NB processors: {}", system.get_processor_list().len() - 1);
}

cfg_if! {
    if #[cfg(unix)] {
        // List of all Components
        pub fn components(system: sysinfo::System) {
            use sysinfo::SystemExt;

            // Linux Only Components:
            for component in system.get_components_list() {
                println!("Components: {:?}", component);
            }
        }
    }
}

// Network Info - Needs to be validated
pub fn network(system: sysinfo::System) {
    use sysinfo::{NetworkExt, SystemExt};

    println!(" Input Data: {} B", system.get_network().get_income());
    println!("Output Data: {} B", system.get_network().get_outcome());
}

#[cfg(test)]
mod unit {
    use super::*;
    use os_info;

    #[test]
    fn print_out_test() {
        let mut return_value: HashMap<String, String> = HashMap::new();
        return_value.insert("String One".to_string(), "String Two".to_string());
        let mut value = Vec::new();
        print_out(return_value, &mut value);
        assert_eq!(value, b"{\"String One\": \"String Two\"}\n");
    }

    #[test]
    fn os_test() {
        let info = os_info::get();
        let return_value = os(info);
        assert!(return_value.contains_key("Version:"), true);
        assert!(return_value.contains_key("Type:"), true);
    }
}
