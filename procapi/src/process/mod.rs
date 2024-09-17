use procapi_core::process::get_processes;
use procapi_core::process::Process;

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub processes: Vec<Process>,
}

impl ProcessInfo {
    pub fn init() -> Self {
        Self {
            processes: get_processes().unwrap(), // пох, пока так
        }
    }
}
