extern crate sys_mount;

use nix::unistd::{chroot, sethostname, chdir, setuid, setgid, Uid, Gid, setresgid, setresuid};
use std::{process, process::Command};
use crate::child::spawn_child;

pub fn init() {
    println!("Initializing container environment");
    chroot("/container").expect("Failed to chroot");
    chdir("/").expect("Failed to chdir");

    let child_args = ["child"].to_vec();

    sys_mount::Mount::new("proc", "proc", "proc", sys_mount::MountFlags::empty(), Some(""));

    spawn_child();
}