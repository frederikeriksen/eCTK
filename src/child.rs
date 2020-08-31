use std::process::Command;
use nix::unistd::sethostname;

pub fn spawn_child() {
    let child_args = ["child"].to_vec();
    Command::new("/proc/self/exe")
        .args(child_args)
        .spawn()
        .expect("Failed to spawn child")
        .wait();
}