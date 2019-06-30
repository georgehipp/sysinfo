use structopt::StructOpt;
mod sysinfo_report;

#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(unix)] {
        #[derive(Debug, StructOpt)]
        #[structopt(name = "SysInfo Report", about = "CLI Tool to list System Information.")]

        struct Options {
            #[structopt(name = "option")]
            /// os | d | mem | proc | cpu | comp | net
            ///
            /// os - List of Operating System Information
            ///
            /// d | disk - List of Disk Information
            ///
            /// mem | memory - System Memory Usage
            ///
            /// proc | processes - Information about all processes on system
            ///
            /// cpu - Number of System Cores
            ///
            /// comp | components - List of System Components
            ///
            /// net | network - Network Usage Statistics
            option: String,
        }
    } else {
        #[derive(Debug, StructOpt)]
        #[structopt(name = "SysInfo", about = "CLI Tool to list System Information.")]

        struct Options {
            #[structopt(name = "option")]
            /// os | d | mem | proc | cpu | comp | net
            ///
            /// os - List of Operating System Information
            ///
            /// d | disk - List of Disk Information
            ///
            /// mem | memory - System Memory Usage
            ///
            /// proc | processes - Information about all processes on system
            ///
            /// cpu - Number of System Cores
            ///
            /// net | network - Network Usage Statistics
            option: String,
        }
    }
}

fn main() {
    call(&Options::from_args().option);

    cfg_if! {
        if #[cfg(unix)] {
            fn call(arg: &str) {
                use sysinfo::SystemExt;
                let mut system = sysinfo::System::new();
                let info = os_info::get();
                let mut value = Vec::new();

                system.refresh_all();

                match arg {
                    "os" => sysinfo_report::print_out(os(info)),
                    "d" | "disk" => sysinfo_report::disk(system),
                    "mem" | "memory" => sysinfo_report::memory(system),
                    "proc" | "processes" => sysinfo_report::processes(system),
                    "cpu" => sysinfo_report::cpu(system),
                    "comp" | "components" => sysinfo_report::components(system),
                    "net" | "network" => sysinfo_report::network(system),
                    _ => println!("{:?} Not a Valid Option", arg),
                }
            }
        } else {
            fn call(arg: &str) {
                use sysinfo::SystemExt;
                let mut system = sysinfo::System::new();
                let info = os_info::get();
                let mut value = Vec::new();

                system.refresh_all();

                match arg {
                    "os" => sysinfo_report::print_out(sysinfo_report::os(info), &mut value),
                    "d" | "disk" => sysinfo_report::disk(system),
                    "mem" | "memory" => sysinfo_report::memory(system),
                    "proc" | "processes" => sysinfo_report::processes(system),
                    "cpu" => sysinfo_report::cpu(system),
                    "net" | "network" => sysinfo_report::network(system),
                    _ => println!("{:?} Not a Valid Option", arg),
                }
            }
        }
    }
}
