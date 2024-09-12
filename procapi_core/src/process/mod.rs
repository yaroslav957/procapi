mod platform;
pub mod state;
mod thread;

use crate::process::state::State;
use crate::process::thread::Thread;
pub use platform::*;

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub cmd: String,
    pub state: State,
    /* pub usage: Usage */
    pub threads: Vec<Thread>,
}
