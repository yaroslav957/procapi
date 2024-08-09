use procapi_sys::process::get_processes;
use procapi_sys::process::Process;


#[derive(Clone)]
pub struct ProcessInfo {
    pub processes: Vec<Process>,
}

impl ProcessInfo {
    pub fn processes() -> Vec<Process> {
        get_processes() // must work 
    }
}