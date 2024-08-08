use procapi_sys::process::Process;
use procapi_sys::process::get_processes;


use libproc::processes::pids_by_type;
use libproc::processes::ProcFilter;

#[derive(Clone)]
pub struct ProcessInfo {
    pub processes: Vec<Process>
}

impl ProcessInfo {
    #[cfg(target_os = "macos")]
    pub fn processes() -> Vec<Process> {
        pids_by_type(ProcFilter::All)
            .unwrap_or_default()
            .iter()
            .filter_map(|&pid| Process::try_from(pid as i32).ok()) // LINUX ERROR
            .collect::<Vec<Process>>()
    }

    pub fn processes() -> Vec<Process> {
        get_processes()
    }
}
