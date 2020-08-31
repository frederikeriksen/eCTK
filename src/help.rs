pub fn help() {
    println!("      ");
    println!("      Run: ectk init, to initialize an environment");
    println!("");
    println!("      Run: ectk unshare, to unshare namespaces");
    println!("      Options: -k, comma-separated list of namespaces to stay in");
    println!("");
    println!("      Run: ectk restrict, to restrict resource usage");
    println!("      Options: -p, max process allowed, -c, max cpu allowed, -m, max memory allowed");
    println!("");
    println!("      Run: ectk wrap, to wrap your created environment in a tarball");
    println!("");
    println!("      Run: ectk unwrap, to unwrap an environment from a tarball");
    println!("      Options: -p, path to file, default is .");
}