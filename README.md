# Artifice Installer - Installer for the Artifice Network

## dependcies
```toml
[dependencies]
installer = "*"
manager = "*"
```

## Example

```rust
use manager::{ArtificeDB, Database};
pub mod installation;
use installer::installation::*;
use std::time::Duration;
use manager::database::ArtificePeers;

fn main() {
    let database = ArtificeDB::create("/home/user/.artifice").unwrap();
    let mut installer = Installer::new(InstallationSrc::NewCompiled, database, 4, Duration::from_secs(5000000));
    let first_task = Task::<std::io::Error, ArtificeDB>::new(1, "create", move |database|{
        let peers: ArtificePeers = database.create_table("peers".to_string(), b"hello_world")?;
        Ok(())
    });
    installer.add_task(first_task);
    installer.run();
}
```