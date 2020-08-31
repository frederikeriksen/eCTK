extern crate sys_mount;
extern crate prctl;
extern crate libc;

use std::{process, process::Command};
use std::str::FromStr;
use nix::sched::{CloneFlags, setns};
use nix::unistd::{chroot, sethostname, chdir, setuid, setgid, Uid, Gid, setresgid, setresuid};
use clap::{ArgMatches};
use std::fs::{Permissions, set_permissions};
use sys_mount::{Mount, MountFlags, SupportedFilesystems};
use nix::errno::Errno;

use crate::namespace::{Namespace, to_cloneflag};
use crate::helpers::is_initialized;
use crate::child::spawn_child;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

pub fn unshare(args: &ArgMatches) {
    if !is_initialized() {
        return;
    }

    let mut namespacesToKeep: Vec<Namespace> = Vec::new();
    if args.args.contains_key("keep") {
        namespacesToKeep = args
        .args
        .get("keep")
        .unwrap()
        .vals
        .clone()
        .iter()
        .map(|val|val.to_str())
        .flat_map(|s| Namespace::from_str(s.unwrap()))
        .collect();
    }

    for namespace in Namespace::iterator() {
        let mut unshare_user: bool = false;
        if !namespacesToKeep.contains(&namespace) {
            let cloneflag = to_cloneflag(&namespace).unwrap();
            if(cloneflag == CloneFlags::CLONE_NEWUSER) {
                unshare_user = true;
            } else {
                println!("Unsharing namespace: {}", &namespace);
                nix::sched::unshare(
                    cloneflag
                ).expect("Failed to unshare namespace");
            }
        }
        if(unshare_user) {
            //setid(Gid::from_raw(10), Uid::from_raw(10)).expect("Failed to set ids");
        }
    }

    spawn_child();
}

fn setid(gid: Gid, uid: Uid) -> Result<()> {
    if let Err(e) = prctl::set_keep_capabilities(true) {
        println!("E");
    };
    {
        setresgid(gid, gid, gid)?; 
    }
    {
        setresuid(uid, uid, uid)?;
    }
    if let Err(e) = prctl::set_keep_capabilities(false) {
        println!("E");
    };
    Ok(())
} 