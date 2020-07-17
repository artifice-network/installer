/*!

this crate serves as the installer for the artifice network, however it can also be used as a simple task scheduler

# Example

```
use manager::{ArtificeDB, Database};
use artifice-installer::installation::*;
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
*/

#[macro_use]extern crate serde_derive;
///
/// this module is where the primary functionality of this crate is located
///
pub mod installation;
/// create and manage tasks (stored in Installer)
pub use installation::TaskSchedule; 
/// installer for the artifice Network
pub use installation::Installer;
/// represents a task to be executed
pub use installation::Task;