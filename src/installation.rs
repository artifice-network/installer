use dirs::*;

use std::fmt;
use std::iter::{IntoIterator, Iterator};
use std::path::{PathBuf, Path};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use networking::peers::ArtificePeer;
use manager::Database;
use std::marker::PhantomData;

/// this structure is a container for a task, or rather an abstraction over a series of steps in the installation process
/// this struct is stored and used by the taskschedule struct, that is responsible for running tasks in parrallel
/// perhaps it would have been easier to use async await I don't know
#[derive(Clone)]
pub struct Task<E: std::error::Error + Send + Sync, DB: Database<E> + Send + Sync> {
    task_id: u16,
    name: String,
    task: Arc<Box<dyn Fn(Arc<DB>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync>>,
    phantom: PhantomData<E>,
}
impl<E: std::error::Error + Send + Sync, DB: Database<E> + Send + Sync> fmt::Debug for Task<E, DB> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            .field("task id", &self.task_id)
            .field("name", &self.name)
            .finish()
    }
}
impl<E: std::error::Error + Send + Sync, DB: Database<E> + Send + Sync> Task<E, DB> {
    /// create a task
    /// # Arguments
    /// task id: some number that identifies a task
    /// name: the name of the task
    /// func: the closure that handles the code that is run in a given task
    ///
    /// # Example
    ///
    /// ```
    /// let task = Task::new(0, "random task", mvoe |db|{
    ///  // run some code in here   
    ///     Ok(())
    /// });
    /// task.run().unwrap();
    /// ```
    pub fn new<F>(task_id: u16, name: &str, func: F) -> Self
    where
        F: Fn(Arc<DB>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + 'static + Send + Sync,
    {
        Self {
            task_id,
            name: name.to_string(),
            task: Arc::new(Box::new(func)),
            phantom: PhantomData,
        }
    }
    pub fn run(
        self,
        database: Arc<DB>,
    ) -> Result<(String, u16), ((String, u16), Box<dyn std::error::Error + Send + Sync>)> {
        let task = self.task;
        match task(database) {
            Ok(()) => Ok((self.name, self.task_id)),
            Err(e) => Err(((self.name, self.task_id), e)),
        }
    }   
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn id(&self) -> u16 {
        self.task_id
    }
}
/// this struct runs the series of jobs or tasks that actually install artifice
/// it uses a sender and receiver to dispatch tasks and recieve the result of each of those tasks post execution
pub struct TaskSchedule<E: 'static + std::error::Error + Send + Sync + Send + Sync, DB: Database<E> + Send + Sync> {
    tasks: Vec<Task<E, DB>>,
    recv:
        Receiver<Result<(String, u16), ((String, u16), Box<dyn std::error::Error + Send + Sync>)>>,
    sender:
        Sender<Result<(String, u16), ((String, u16), Box<dyn std::error::Error + Send + Sync>)>>,
    thread_count: u8,
    timeout: Duration,
}
impl<E: 'static + std::error::Error + Send + Sync, DB: 'static + Database<E> + Send + Sync> TaskSchedule<E, DB> {
    /// allows a task list to be constructed from existing tasks, as appose to creating an empty list
    /*pub fn from_tasks(task_list: &[Task<E, DB>], thread_count: u8, timeout: Duration) -> Self {
        let (sender, recv): (
            Sender<
                Result<(String, u16), ((String, u16), Box<dyn std::error::Error + Send + Sync>)>,
            >,
            Receiver<
                Result<(String, u16), ((String, u16), Box<dyn std::error::Error + Send + Sync>)>,
            >,
        ) = channel();
        let mut tasks = Vec::new();
        tasks.extend_from_slice(task_list);
        Self {
            tasks,
            recv,
            sender,
            thread_count,
            timeout,
        }
    }*/
    pub fn new(thread_count: u8, timeout: Duration) -> Self {
        let tasks = Vec::new();
        let (sender, recv): (
            Sender<
                Result<(String, u16), ((String, u16), Box<dyn std::error::Error + Send + Sync>)>,
            >,
            Receiver<
                Result<(String, u16), ((String, u16), Box<dyn std::error::Error + Send + Sync>)>,
            >,
        ) = channel();
        Self {
            tasks,
            recv,
            sender,
            thread_count,
            timeout,
        }
    }
    pub fn add_task(&mut self, task: Task<E, DB>) {
        self.tasks.push(task);
    }
    /// consumes the task list and dispateches $thread_count threads to run each task
    pub fn run(
        mut self,
        database: Arc<DB>,
    ) -> Result<(String, u16), ((String, u16), Box<dyn std::error::Error + Send + Sync>)> {
        let mut tasks_complete = 0;
        let tasks_total = self.tasks.len();
        let sender = self.sender.clone();
        for _ in 0..self.thread_count {
            if let Some(task) = self.tasks.pop() {
                println!("starting task: {} with id: {}", task.name(), task.id());
                let newsender = sender.clone();
                let db = database.clone();
                thread::spawn(move || {
                    newsender.send(task.run(db)).unwrap();
                });
            }
        }
        while let Ok(result) = self.recv.recv_timeout(self.timeout) {
            match result {
                Ok((name, id)) => {
                    tasks_complete += 1;
                    println!(
                        "task: {}, with id: {} completed successfully, {}/{}",
                        name, id, tasks_complete, tasks_total
                    );
                    if let Some(task) = self.tasks.pop() {
                        println!("starting task: {} with id: {}", task.name(), task.id());
                        let nextsender = self.sender.clone();
                        let db = database.clone();
                        thread::spawn(move || {
                            nextsender.send(task.run(db)).unwrap();
                        });
                    } else {
                        return Ok(("".to_string(), 0));
                    }
                }
                Err(e) => return Err(e),
            }
        }
        Ok(("".to_string(), 0))
    }
}
/// selects wether to compile the code from source or use precompiled binaries
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum InstallationSrc{
    NewCompiled,
    NewFromSrc,
}
/// allows for the selection of varying locations for configs,
/// configs in this case being ArtificePeers see networking, ArtificeConfig, and ArtificeHost
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ConfigSrc{
    Peer(ArtificePeer),
    CompletePeer(ArtificePeer),
    UpStream(String),
    Empty,
}
/// this struct is used for the operation of the installation process
pub struct Installer<E: 'static + std::error::Error + Send + Sync, DB: 'static + Database<E> + Send + Sync> {
    install_src: InstallationSrc,
    schedule: TaskSchedule<E, DB>,
    config_src: ConfigSrc,
    installation: DB,
    error: std::marker::PhantomData<E>,
}

impl<E: std::error::Error + Send + Sync, DB: Database<E> + Send + Sync> Installer<E, DB> {
    ///
    /// # Arguments
    /// install_src: precompiled, versus compiled
    /// database: the installation destination, ideally will be passed into the callbacks of each task
    /// thread count: how many threads to run tasks on in parrellel (means tasks can't be sequential)
    /// task_duration how long should the task schedule wait before killing a task because it has hung
    /// 
    /// # Example 
    ///
    /// ```
    /// //note if not in this location any attempt to start the network will fail, because that is the current place that the system will look for configs
    /// // perhaps in the future the configs will be set to load from an env var
    /// let database = ArtificeDB::create("/home/user/.artifice");
    ///
    /// let mut installer = Installer::new(InstalllationSrc::NewCompiled, database, 5, Durattion::from_seconds(5000000));
    ///
    /// let task = Task::new(0, "create_dirs", move |db| {
    ///     //some code here to preform a task
    /// });
    ///
    /// installer.add_task(task);
    /// installer.run();
    /// ```
    ///
    pub fn new(install_src: InstallationSrc, database: DB, thread_count: u8, task_duration: Duration) -> Self{
        let schedule = TaskSchedule::new(thread_count, task_duration);
        Self {install_src, schedule, config_src: ConfigSrc::Empty, installation: database, error: std::marker::PhantomData}
    }
    /// update the config src from generating new config to using existing configs
    pub fn config_src(mut self, config_src: ConfigSrc) -> Self{
        self.config_src = config_src;
        self
    }
    /// adds a task to the task schedule
    pub fn add_task(&mut self, task: Task<E, DB>){
        self.schedule.add_task(task);
    }
    /// used to run the constructed task schedule that performs each step needed for installation
    pub fn run(self) -> Result<(String, u16), ((String, u16), Box<dyn std::error::Error + Send + Sync>)>{
        self.schedule.run(Arc::new(self.installation))
    }
}