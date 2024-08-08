use std::{
    io::{Error, ErrorKind},
};

use libproc::proc_pid;
use libproc::proc_pid::{ListThreads, pidpath};

#[cfg(target_os = "macos")]
use libproc::{
    task_info::TaskAllInfo,
    proc_pid::listpidinfo,
    thread_info::ThreadInfo,
    proc_pid::pidinfo,
};

#[derive(Debug, Clone)]
pub struct Process {
    ids: [u32; 2],
    name: String,
    state: State,
    cmd: String,
    //usage
}

#[derive(Debug, Clone)]
pub struct State(RunState);

#[derive(Debug, Clone)]
pub enum RunState {
    Idle,
    Runnable,
    Sleeping,
    Stopped,
    UninterruptibleWait,
    Dead,
}

impl TryFrom<&str> for State {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(State(RunState::try_from(s.as_bytes()[0] as char)
            .unwrap_or(RunState::Dead)))
    }
}

impl TryFrom<char> for RunState {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'I' => Self::Idle,
            'R' => Self::Runnable,
            'S' => Self::Sleeping,
            'T' => Self::Stopped,
            'U' => Self::UninterruptibleWait,
            'Z' => Self::Dead,
            _ => return Err(
                Error::new(ErrorKind::InvalidInput, "Invalid state")
            )
        })
    }
}

impl Process {
    pub(crate) fn parse_state(pth_state: u32) -> char {
        match pth_state {
            1 => 'R',
            2 => 'U',
            3 => 'S',
            4 => 'I',
            5 => 'T',
            6 => 'H',
            _ => '?',
        }
    }
}

#[cfg(target_os = "macos")]
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

            Ok(Process {
                ids: [pid as u32, info.pbsd.pbi_ppid],
                name: proc_pid::name(pid).unwrap_or_else(|_| {
                    pidpath(pid).unwrap_or_default()
                }),
                state: State(RunState::try_from(Self::parse_state(pth_state))?),
                cmd: pidpath(pid).unwrap_or_default(),
            })
        } else {
            Err(Error::last_os_error())
        }
    }
}