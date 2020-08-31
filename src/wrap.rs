use std::fs::File;
use std::fs::DirBuilder;
use std::os::unix::fs::DirBuilderExt;
use flate2::Compression;
use flate2::write::GzEncoder;
use clap::ArgMatches;
use tar;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

pub fn wrap(args: &ArgMatches) {
    let tar_dir = DirBuilder::new()
        .create("containerimage")
        .mode(0o777)
        .expect("Failed to create tar dir");

    let tar_gz = File::create("/containerimage/archive.tar.gz")
        .expect("Failed to create tar");
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("./", "./containerimage")
        .expect("Failed to copy files to tarball");
    
}