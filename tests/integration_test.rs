    
extern crate assert_cli;
#[macro_use]
extern crate cfg_if;

#[cfg(test)]
mod integration {
    use assert_cli;

    #[test]
    fn os_test() {
        assert_cli::Assert::command(&["cargo", "run", "--", "os"])
        .stdout().contains("Type:")
        .stdout().contains("Version:")
        .unwrap();
    }

    #[test]
    fn disk_test() {
        assert_cli::Assert::command(&["cargo", "run", "--", "disk"])
        .stdout().contains("Disk")
        .unwrap();
    }

    #[test]
    fn memory_test() {
        assert_cli::Assert::command(&["cargo", "run", "--", "mem"])
        .stdout().contains("Memory:")
        .unwrap();
    }

    #[test]
    fn processes_test() {
        assert_cli::Assert::command(&["cargo", "run", "--", "proc"])
        .stdout().contains("Process")
        .unwrap();
    }

    #[test]
    fn cpu_test() {
        assert_cli::Assert::command(&["cargo", "run", "--", "cpu"])
        .stdout().contains("processors:")
        .unwrap();
    }

    cfg_if! {
        if #[cfg(unix)] {
            #[test]
            fn components_test() {
                assert_cli::Assert::command(&["cargo", "run", "--", "comp"])
                .stdout().contains("")
                .unwrap();
            }
        }
    }

    #[test]
    fn network_test() {
        assert_cli::Assert::command(&["cargo", "run", "--", "net"])
        .stdout().contains("Data")
        .unwrap();
    }

    #[test]
    fn invalide_option_test() {
        assert_cli::Assert::command(&["cargo", "run", "--", "invalid"])
        .stdout().contains("Not a Valid Option")
        .unwrap();
    }
}