#[macro_use] extern crate failure;
extern crate sys_mount;

mod namespace;
mod unshare;
mod restrict;
mod init;
mod help;
mod helpers;
mod child;
mod wrap;
//mod unwrap;

use nix::sched::{CloneFlags, setns};
use nix::unistd::{chroot, sethostname, chdir, setuid, setgid, Uid, Gid, setresgid, setresuid};
use std::{path::Path, process, process::Command};
use clap::{App, Arg, SubCommand, ArgMatches};

use namespace::Namespace;
use unshare::unshare;
use restrict::restrict;
use init::init;
use wrap::wrap;
use help::help;

fn main() {
    std::env::set_var("RUST_BACKTRACT", "1");
    let version: &str = env!("CARGO_PKG_VERSION");
    let author: &str = env!("CARGO_PKG_AUTHORS");
    let about: &str = "ectk is a simplified cli toolkit for learning about container constructs";

    let app: App = App::new("ectk")
        .version(version)
        .author(author)
        .about(about)
        .subcommand(SubCommand::with_name("init")
            .help("Initialized the container environment"))
        .subcommand(SubCommand::with_name("wrap")
            .help("Wraps environment in tarball"))   
        .subcommand(SubCommand::with_name("unshare")
            .about("Unshare given namespaces")
            .arg(Arg::with_name("config")
                .short("-c")
                .long("--config")
                .help("Path to config file")
                .takes_value(true))
            .arg(Arg::with_name("keep")
                .short("-k")
                .long("--keep")
                .help("Namespaces to keep")
                .min_values(1)
                .max_values(7)
                .use_delimiter(true)))
        .subcommand(SubCommand::with_name("restrict")
            .about("Restricts desired resources")
            .arg(Arg::with_name("pidmax")
                .short("-p")
                .long("--pidmax")
                .help("Restrict max pids")
                .takes_value(true)
                .number_of_values(1))
            .arg(Arg::with_name("cpu")
                .short("-c")
                .long("--cpu")
                .help("Restrict cpu usage")
                .takes_value(true)
                .number_of_values(1))
            .arg(Arg::with_name("memory")
                .short("-m")
                .long("--memory")
                .help("Restrict memory usage")
                .takes_value(true)
                .max_values(1)))
        .subcommand(SubCommand::with_name("child")
            .about("Creates the contained process. Must be called by calling run.")
            .arg(Arg::with_name("config")
                .short("-c")
                .long("--config")
                .help("Path to config file")
                .takes_value(true)
                .default_value("."))
            .arg(Arg::with_name("unshare")
                .short("-u")
                .long("--unshare")
                .help("Unshare namespaces")
                .min_values(1)
                .max_values(7)
                .use_delimiter(true)))
        .subcommand(SubCommand::with_name("help")
            .about("Returns help information"));

    let matches = app.get_matches();
    
    match matches.subcommand() {
        ("init", Some(init_matches)) => init(),
        ("unshare", Some(unshare_matches)) => unshare(unshare_matches),
        ("restrict", Some(restrict_matches)) => restrict(restrict_matches),
        ("wrap", Some(wrap_matches)) => wrap(wrap_matches),
        ("child", Some(child_matches)) => child(child_matches),
        ("help", Some(help_matches)) => help(),
        _ => panic!("Command not recognized")
    }
}

fn child(args: &ArgMatches) {
    println!("Running with pid {}", process::id());
    let process_id = process::id();
    sethostname("container");

    sys_mount::Mount::new("proc", "proc", "proc", sys_mount::MountFlags::empty(), Some(""));
    
    Command::new("/bin/bash")
        .spawn()
        .expect("Failed")
        .wait();
}
