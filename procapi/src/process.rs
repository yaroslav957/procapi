use procapi_sys::ProcessInfo;
use procapi_sys::get_process_list;

#[derive(Debug)]
pub struct Process {
    ids: [u32; 2],
    name: String,
    state: State,
    cmd: String,
    //usage
}

impl From<ProcessInfo> for Process {
    fn from(info: ProcessInfo) -> Self {
        Self {
            ids: [info.pid, info.ppid],
            name: info.name,
            state: State {
                run_state: RunState::from_char(info.run_state)
                    .unwrap_or(RunState::Dead)
            },
            cmd: info.cmd
        }
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
}

#[cfg(unix)]
pub fn processes_info() -> Vec<Process> {
    get_process_list()
        .iter()
        .map(|pinfo| Process::from(pinfo.clone()))
        .collect::<Vec<Process>>()
}
