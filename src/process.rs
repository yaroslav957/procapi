use std::process::Command;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug)]
pub struct Process {
    ids: [u32; 2],
    name: String,
    state: ProcessState,
    cmd: String,
    //usage
}

#[derive(Debug)]
pub struct ProcessState {
    run_state: ProcessRunState,
    // extra
}

impl ProcessState {
    pub fn from_str(s: &str) -> Self {
        ProcessState {
            run_state: ProcessRunState::from_char(s.as_bytes()[0] as char)
                .unwrap_or(ProcessRunState::Dead)
        }
    }
}

#[derive(Debug)]
pub enum ProcessRunState {
    Idle,
    Runnable,
    Sleeping,
    Stopped,
    UninterruptibleWait,
    Dead,
}

impl ProcessRunState {
    pub fn from_char(c: char) -> Option<ProcessRunState> {
        Some(match c {
            'I' => ProcessRunState::Idle,
            'R' => ProcessRunState::Runnable,
            'S' => ProcessRunState::Sleeping,
            'T' => ProcessRunState::Stopped,
            'U' => ProcessRunState::UninterruptibleWait,
            'Z' => ProcessRunState::Dead,
            _ => return None
        })
    }
}

#[cfg(unix)]
pub fn processes_info() -> Result<Vec<Process>, std::io::Error> {
    let output = Command::new("ps")
        .arg("-eo")
        .arg("pid,ppid,state,command")
        .output()?;
    let output = String::from_utf8_lossy(&output.stdout);
    let processes = output.lines()
        .skip(1)
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            if let Some((pid, ppid, state, name)) = parts.next_tuple() {
                let pid = u32::from_str(pid).unwrap_or(0);
                let ppid = u32::from_str(ppid).unwrap_or(0);
                let cmd = parts.join(" ");
                Some(Process {
                    ids: [pid, ppid],
                    name: name.to_string(),
                    state: ProcessState::from_str(state),
                    cmd
                })
            } else { None }
        })
        .collect();

    Ok(processes)
}
