use procapi_core::process::get_processes;
use procapi_core::process::Process;

#[derive(Clone)]
pub struct ProcessInfo {
	pub processes: Vec<Process>,
}

impl ProcessInfo {
	pub fn processes() -> Vec<Process> {
		get_processes() // так как трайформ можт наебнуца, обернуть бы выхлоп `get_processes` в рзелатик, но думаю пока похуй? оно и так небольшое, потом есчо
	}
}
