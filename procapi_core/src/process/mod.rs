mod platform;
pub mod state;

use crate::process::state::State;
pub use platform::*;

#[derive(Debug, Clone)]
pub struct Process {
    pub pid: u32,
    pub ppid: u32,
    // threads
    pub name: String,
    pub cmd: String,
    pub state: State,
    // usage
}
