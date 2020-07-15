use std::path::Path;
use tar::{Builder, Archive};
use walkdir::WalkDir;
use libflate::gzip::{Encoder, Decoder};
use std::io::{Write, Read};

/// this function currently doesn't work hands on write
pub fn compress(archive: &Vec<u8>) -> std::io::Result<Vec<u8>> {
    let mut encoder = Encoder::new(Vec::new())?;
    println!("encoder created, data len: {}", archive.len());
    println!("write: {}", encoder.write(&archive)?);
    println!("data written");
    encoder.finish().into_result()
}
pub fn decompress(encoded_data: &[u8]) -> std::io::Result<Vec<u8>>{
    let mut decoder = Decoder::new(encoded_data)?;
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data)?;
    Ok(decoded_data)
}
pub fn archive(root: &str) -> std::io::Result<Vec<u8>> {
    let archive = Vec::new();
    let mut builder = Builder::new(archive);
    let results: Vec<std::io::Result<()>> = WalkDir::new(root)
        .into_iter()
        .map(|p| p.unwrap().into_path())
        .filter(|p| p.is_file())
        .map(|p| builder.append_path(p))
        .collect();
    for result in results.into_iter() {
        result?
    }
    builder.into_inner()
}
pub fn dearchive<P: AsRef<Path>>(archive_data: &[u8], root: P) -> std::io::Result<()> {
    let mut archive = Archive::new(archive_data);
    for entry in archive.entries()? {
        println!("path: {}", entry?.path()?.display());
    }
    Ok(())
}
pub fn download() {}

//thoughts for the future when copying over remote configs an option for complete copy will be available with risks
//this must be concented to and when it occures the configs from the old system will be purged
//despite this the typical remote config pul will simply pull peer information, permissions, and applications 
//invalidating all pair keys, and regenerating rsa keys
