#[macro_use]
extern crate serde_derive;

use manager::{ArtificeDB, Database};
pub mod installation;
use installation::*;
use std::time::Duration;
use manager::database::ArtificePeers;
use std::io::{Read, Write};
use networking::ArtificeConfig;

fn get_password() -> std::io::Result<String>{
    let mut first_key: [u8;65535] = [0;65535];
    let mut second_key: [u8;65535] = [0;65535];
    std::io::stdout().lock().write(b"enter encryption key: ")?;
    std::io::stdout().lock().flush();
    std::io::stdin().lock().read(&mut first_key)?;
    std::io::stdout().lock().write(b"retype key: ")?;
    std::io::stdout().lock().flush();
    std::io::stdin().lock().read(&mut second_key)?;
    let fkey = first_key.to_vec();
    let skey = second_key.to_vec();
    if skey != fkey {
        println!("keys didn't match");
        return get_password();
    }
    Ok(String::from_utf8(fkey).unwrap())
}

fn main() {
    let database = ArtificeDB::create("/home/user/.artifice").unwrap();
    let password = get_password().unwrap();
    let mut installer = Installer::new(InstallationSrc::NewCompiled, database, 4, Duration::from_secs(5000000));
    let first_task = Task::<std::io::Error, ArtificeDB>::new(1, "create", move |database, schedule|{
        let peers: ArtificePeers = database.create_table("peers".to_string(), &password.clone().into_bytes())?;
        let config: ArtificeConfig = database.load_entry("config".to_string(), &password.clone().into_bytes())?;
        Ok(())
    });
    installer.add_task(first_task);
    installer.run();
}
