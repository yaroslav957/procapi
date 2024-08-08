use libproc::processes::pids_by_type;
use libproc::processes::ProcFilter;
use libproc::proc_pid;
use libproc::proc_pid::pidpath;
use libproc::task_info::TaskAllInfo;
use libproc::proc_pid::ListThreads;
use libproc::proc_pid::listpidinfo;
use libproc::thread_info::ThreadInfo;
use libproc::proc_pid::pidinfo;

#[derive(Debug)]
pub struct Process {
    ids: [u32; 2],
    name: String,
    state: State,
    cmd: String,
    //usage
}

#[cfg(target_os = "macos")]
impl Process {
    pub fn from_pid(pid: i32) -> Option<Self> {
        if let Ok(info) = proc_pid::pidinfo::<TaskAllInfo>(pid, 0) {
            let pth_run_state = listpidinfo::<ListThreads>(
                pid,
                info.ptinfo.pti_threadnum as usize)
                .unwrap_or_default()
                .iter()
                .filter_map(|t| pidinfo::<ThreadInfo>(pid, *t).ok())
                .map(|t| t.pth_run_state)
                .min()
                .unwrap_or(7);

            return Some(Process {
                ids: [pid as u32, info.pbsd.pbi_ppid],
                name: proc_pid::name(pid).unwrap_or_else(|_| {
                    pidpath(pid).unwrap_or_default()
                }),
                state: State {
                    run_state: RunState::from_bsd_run_state(thread_run_state)
                        .unwrap_or(RunState::Dead),
                },
                cmd: pidpath(pid).unwrap_or_default()
            })
        }

        None
    }
}

#[derive(Debug)]
pub struct State {
    run_state: RunState,
}

impl State {
    pub fn from_str(s: &str) -> Self {
        State {
            run_state: RunState::from_char(s.as_bytes()[0] as char)
                .unwrap_or(RunState::Dead),
        }
    }
}

#[derive(Debug)]
pub enum RunState {
    Idle,
    Runnable,
    Sleeping,
    Stopped,
    UninterruptibleWait,
    Dead,
}

impl RunState {
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            'I' => Self::Idle,
            'R' => Self::Runnable,
            'S' => Self::Sleeping,
            'T' => Self::Stopped,
            'U' => Self::UninterruptibleWait,
            'Z' => Self::Dead,
            _ => return None
        })
    }

    #[cfg(target_os = "macos")]
    pub fn from_bsd_run_state(pth_run_state: i32) -> Option<Self> {
        let c = match thread_run_state {
            1 => 'R',
            2 => 'U',
            3 => 'S',
            4 => 'I',
            5 => 'T',
            6 => 'H',
            _ => '?',
        };

        Self::from_char(c)
    }
}

#[cfg(unix)]
pub fn processes_info() -> Vec<Process> {
    pids_by_type(ProcFilter::All)
        .unwrap_or_default()
        .iter()
        .filter_map(|pid| Process::from_pid(*pid as i32))
        .collect::<Vec<Process>>()
}
