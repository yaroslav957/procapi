use procapi_core::process::get_processes;
use procapi_core::process::Process;

#[derive(Clone)]
pub struct ProcessInfo {
	pub processes: Vec<Process>,
}

impl ProcessInfo {
	pub fn processes() -> Vec<Process> {
		get_processes()
	}
}
