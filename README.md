# Artifice Installer - Installer for the Artifice Network

## description
this crate serves as the installer for the artifice network, however it can also be used as a simple task scheduler

## dependcies
```toml
[dependencies]
installer = "*"
manager = "*"
```

## Example

```rust
use manager::{ArtificeDB, Database};
use installer::installation::*;
use std::time::Duration;
use manager::database::ArtificePeers;
use std::io::{Read, Write};
use networking::ArtificeConfig;

fn main(){
    let database = ArtificeDB::create("/home/user/.artifice").unwrap();
    let password = "hello_world".to_string();
    let mut installer = Installer::new(InstallationSrc::NewCompiled, database, 4, Duration::from_secs(5000000));
    let first_task = Task::<std::io::Error, ArtificeDB>::new(1, "create", move |database, schedule|{
        let peers: ArtificePeers = database.create_table("peers".to_string(), &password.clone().into_bytes())?;
        let config: ArtificeConfig = database.load_entry("config".to_string(), &password.clone().into_bytes())?;
        Ok(())
    });
    installer.add_task(first_task);
    installer.run();
}
```