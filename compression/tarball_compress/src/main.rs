
use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

fn main() -> Result<(), std::io::Error> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    let mut f = File::open("hi.txt").unwrap();
    tar.append_file("hi.txt", &mut f)?;
    Ok(())
}
