use libproc::processes::pids_by_type;
use libproc::processes::ProcFilter;
use libproc::proc_pid;
use libproc::proc_pid::pidpath;
use libproc::task_info::TaskAllInfo;
use libproc::proc_pid::ListThreads;
use libproc::proc_pid::listpidinfo;
use libproc::thread_info::ThreadInfo;
use libproc::proc_pid::pidinfo;

use super::process::ProcessInfo;

impl ProcessInfo {
    pub fn from_pid(pid: i32) -> Option<Self> {
        if let Ok(info) = proc_pid::pidinfo::<TaskAllInfo>(pid, 0) {
            let pth_run_state = listpidinfo::<ListThreads>(
                pid,
                info.ptinfo.pti_threadnum as usize)
                .unwrap_or_default()
                .iter()
                .filter_map(|&t| pidinfo::<ThreadInfo>(pid, t).ok())
                .map(|t| match t.pth_run_state {
                    1 => 1, // TH_STATE_RUNNING
                    2 => 5, // TH_STATE_STOPPED
                    3 => {  // TH_STATE_WAITING
                        if t.pth_sleep_time > 20 { 4 } else { 3 }
                     }
                    4 => 2, // TH_STATE_UNINTERRUPTIBLE
                    5 => 6, // TH_STATE_HALTED
                    _ => 7,
                })
                .min()
                .unwrap_or(7);

            return Some(ProcessInfo {
                pid: pid as u32,
                ppid: info.pbsd.pbi_ppid,
                name: proc_pid::name(pid).unwrap_or_else(|_| {
                    pidpath(pid).unwrap_or_default()
                }),
                run_state: parse_bsd_run_state(pth_run_state),
                cmd: pidpath(pid).unwrap_or_default()
            })
        }

        None
    }
}

fn parse_bsd_run_state(pth_run_state: u32) -> char {
    match pth_run_state {
        1 => 'R',
        2 => 'U',
        3 => 'S',
        4 => 'I',
        5 => 'T',
        6 => 'H',
        _ => '?',
    }
}

pub fn get_process_list() -> Vec<ProcessInfo> {
    pids_by_type(ProcFilter::All)
        .unwrap_or_default()
        .iter()
        .filter_map(|&pid| ProcessInfo::from_pid(pid as i32))
        .collect::<Vec<ProcessInfo>>()
}
