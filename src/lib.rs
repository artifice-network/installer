//!
//!
//! this crate serves as the installer for the artifice network, however it can also be used as a simple task scheduler
//!
//! # Example
//!
//! ```
//!
//!use manager::{ArtificeDB, Database};
//!use installer::installation::*;
//!use std::time::Duration;
//!use manager::database::ArtificePeers;
//!
//! fn main() {
//!     // this uses the default path to artifice, currently any other path will cause a crash when the host is started
//!     let database = ArtificeDB::create("/home/user/.artifice").unwrap();
//!     let mut installer = Installer::new(InstallationSrc::NewCompiled, database, 4, Duration::from_secs(5000000));anything that implements std::error::Error can be returned from it
//! 
//!     // the return type of the closure is a box dyn error so any result with a unt or () can be returned
//!
//!     let first_task = Task::<std::io::Error, ArtificeDB>::new(1, "create", move |database|{
//!        let peers: ArtificePeers = database.create_table("peers".to_string(), b"hello_world")?;
//!        Ok(())
//!     });
//! 
//!     installer.add_task(first_task);
//!     installer.run();
//! }
//!
//! ```

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