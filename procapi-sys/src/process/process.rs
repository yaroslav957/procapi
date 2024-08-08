#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub run_state: char,
    pub name: String,
    pub cmd: String
}

#[cfg(target_os = "macos")]
pub use crate::process::macos::get_process_list;
