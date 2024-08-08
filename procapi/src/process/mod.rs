use procapi_sys::process::Process;

use libproc::processes::pids_by_type;
use libproc::processes::ProcFilter;

#[derive(Clone)]
pub struct ProcessInfo {
    pub processes: Vec<Process>
}

impl ProcessInfo {
    #[cfg(unix)]
    pub fn processes() -> Vec<Process> {
        pids_by_type(ProcFilter::All)
            .unwrap_or_default()
            .iter()
            .filter_map(|&pid| Process::try_from(pid as i32)) // LINUX ERROR
            .collect::<Vec<Process>>()
    }
}