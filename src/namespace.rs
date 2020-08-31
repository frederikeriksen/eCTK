use nix::sched::{CloneFlags};
use std::str::FromStr;
use std::{fmt, fmt::Display, fmt::Formatter};
use self::Namespace::*;
use std::slice::Iter;

#[derive(Debug, PartialEq)]
pub enum Namespace {
    PID,
    UTS,
    USER,
    MOUNT,
    NET,
    IPC,
    CGROUP
}

impl Namespace {
    pub fn iterator() -> Iter<'static, Namespace> {
        static NAMESPACES: [Namespace; 7] = [PID, UTS, USER, MOUNT, NET, IPC, CGROUP];
        NAMESPACES.iter()
    }
}

impl FromStr for Namespace {
    type Err = ();

    fn from_str(string: &str) -> Result<Namespace, ()> {
        match string {
            "pid" => Ok(Namespace::PID),
            "uts" => Ok(Namespace::UTS),
            "user" => Ok(Namespace::USER),
            "mount" => Ok(Namespace::MOUNT),
            "net" => Ok(Namespace::NET),
            "ipc" => Ok(Namespace::IPC),
            "cgroup" => Ok(Namespace::CGROUP),
            _ => Ok(Namespace::PID)
        }
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}


pub fn to_cloneflag(namespace: &Namespace) -> Result<CloneFlags, ()> {
    match namespace {
        Namespace::PID => Ok(CloneFlags::CLONE_NEWPID),
        Namespace::UTS => Ok(CloneFlags::CLONE_NEWUTS),
        Namespace::USER => Ok(CloneFlags::CLONE_NEWUSER),
        Namespace::MOUNT => Ok(CloneFlags::CLONE_NEWNS),
        Namespace::NET => Ok(CloneFlags::CLONE_NEWNET),
        Namespace::IPC => Ok(CloneFlags::CLONE_NEWIPC),
        Namespace::CGROUP => Ok(CloneFlags::CLONE_NEWCGROUP)
    }
}