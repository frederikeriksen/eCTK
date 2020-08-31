use std::path::{Path, PathBuf};
use std::fs::{self, DirBuilder, Permissions, set_permissions};
use std::os::unix::fs::DirBuilderExt;
use clap::{ArgMatches};
use crate::child::spawn_child;

const CGROUP_DEFAULT_PATH: &str = "/sys/fs/cgroup/";

pub fn restrict(args: &ArgMatches) {

    if args.args.contains_key("pidmax") {
        let maxpids: &str = args
            .args
            .get("pidmax")
            .unwrap()
            .vals
            .get(0)
            .map(|s|s.to_str())
            .unwrap()
            .unwrap();

        set_pidmax(maxpids);
    }   

    if args.args.contains_key("cpu") {
        let cpu: &str = args
            .args
            .get("cpu")
            .unwrap()
            .vals
            .get(0)
            .map(|s|s.to_str())
            .unwrap()
            .unwrap();

        set_cpu(cpu);
    }

    if args.args.contains_key("memory") {
        let memory: &str = args
            .args
            .get("memory")
            .unwrap()
            .vals
            .get(0)
            .map(|s|s.to_str())
            .unwrap()
            .unwrap();

        set_memory(memory);
    }
}

fn set_pidmax(max_pids: &str) {
    let mut cgroups = PathBuf::from(&CGROUP_DEFAULT_PATH);
    assert!(cgroups.exists(), "There is no cgroup directory");
    cgroups.push("pids");
    assert!(cgroups.exists(), "Failed to create pids subdirectory");
    cgroups.push("container");

    if !cgroups.exists() { 
        DirBuilder::new()
            .mode(0o777)
            .create(&cgroups)
            .expect("Failed to create cgroups");
    } else {
        println!("Max pids successfully set to {}", &max_pids);
    }
    let pidmax = cgroups.join("pids.max");
    fs::write(pidmax, &max_pids.as_bytes());

    spawn_child();
}

fn set_cpu(max_cpu: &str) {
    let mut cgroups = PathBuf::from(&CGROUP_DEFAULT_PATH);
    assert!(cgroups.exists(), "There is no cgroup directory");
    cgroups.push("cpu");
    assert!(cgroups.exists(), "Failed to create pids subdirectory");
    cgroups.push("container");

    if !cgroups.exists() { 
        DirBuilder::new()
            .mode(0o777)
            .create(&cgroups)
            .expect("Failed to create cgroups");
    }
    let cpu_shares = cgroups.join("cpu.shares");
    fs::write(cpu_shares, &max_cpu.as_bytes());
    println!("Max cpu shares successfully set to {}", &max_cpu);

    spawn_child();
}

fn set_memory(max_memory: &str) {
    let mut cgroups = PathBuf::from(&CGROUP_DEFAULT_PATH);
    assert!(cgroups.exists(), "There is no cgroup directory");
    cgroups.push("cpu");
    assert!(cgroups.exists(), "Failed to create pids subdirectory");
    cgroups.push("container");

    if !cgroups.exists() { 
        DirBuilder::new()
            .mode(0o777)
            .create(&cgroups)
            .expect("Failed to create cgroups");
    }
    let memory_max = cgroups.join("memory.max_usage_in_bytes");
    fs::write(memory_max, &max_memory.as_bytes());
    println!("Memory usage successfully set to {}", &max_memory);
    
    spawn_child();
}