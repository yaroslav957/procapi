use std::io::{Error, ErrorKind};
use libproc::{
    processes::{
        ProcFilter,
        pids_by_type
    },
    task_info::TaskAllInfo,
    thread_info::ThreadInfo,
    proc_pid::listpidinfo,
    proc_pid::pidinfo,
    libproc::proc_pid,
    libproc::proc_pid::{ListThreads, pidpath},
};

use crate::process::{Process, State};

pub fn get_processes() -> Vec<Process> {
    pids_by_type(ProcFilter::All)
        .unwrap_or_default()
        .iter()
        .filter_map(|&pid| Process::try_from(pid as i32).ok())
        .collect::<Vec<Process>>()
}

impl TryFrom<i32> for Process {
    type Error = Error;

    fn try_from(pid: i32) -> Result<Self, Self::Error> {
        if let Ok(info) = proc_pid::pidinfo::<TaskAllInfo>(pid, 0) {
            let pth_state = listpidinfo::<ListThreads>(
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

            dbg!(info.pbsd);

            Ok(Process {
                ids: [pid as u32, info.pbsd.pbi_ppid],
                name: proc_pid::name(pid).unwrap_or_else(|_| {
                    pidpath(pid).unwrap_or_default()
                }),
                state: State::try_from(pth_state)?,
                cmd: pidpath(pid).unwrap_or_default(),
            })
        } else {
            Err(Error::last_os_error())
        }
    }
}

impl TryFrom<u32> for State {
    type Error = Error;

    fn try_from(pth_state: u32) -> Result<Self, Self::Error> {
        Ok(match pth_state {
            1 => State::Runnable,
            2 => State::UninterruptibleWait,
            3 => State::Sleeping,
            4 => State::Idle,
            5 => State::Stopped,
            6 => State::Dead,
            _ => return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unknown pth state: {pth_state}")
            ))
        })
    }
}
