use std::{path::Path};

pub fn is_initialized() -> bool {
    let not_initialized: bool = Path::new("../../container").exists();
    if not_initialized {
        println!("      Container environment not initialized");
        println!("");
        println!("      Did you forget to run: ectk init?");
        println!("");
    }
    return !not_initialized;
}