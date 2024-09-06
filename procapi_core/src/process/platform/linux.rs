use crate::process::{Process, State};
use std::fs;
use std::io::Error;
use std::path::Path;

pub fn get_processes() -> Vec<Process> {
    let mut pids = Vec::new();

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if let Ok(pid) = entry
                        .file_name()
                        .into_string()
                        .unwrap_or_default()
                        .parse::<u32>()
                    {
                        pids.push(pid);
                    }
                }
            }
        }
    }

    pids.iter()
        .filter_map(|&pid| Process::try_from(pid).ok())
        .collect::<Vec<Process>>()
}

impl TryFrom<u32> for Process {
    type Error = Error;

    fn try_from(pid: u32) -> Result<Self, Self::Error> {
        let proc_dir = Path::new("/proc");
        let pid_dir = proc_dir.join(pid.to_string());
        let mut pid = u32::default();
        let mut ppid = u32::default();
        let name = fs::read_to_string(&pid_dir.join("comm"))?
            .trim()
            .to_string();
        let cmd = fs::read_to_string(&pid_dir.join("cmdline"))?.replace('\0', " ");
        let mut state = State::default();

        let _ = fs::read_to_string(&pid_dir.join("status")).map(|status_content| {
            status_content.lines().for_each(|line| {
                if line.starts_with("Pid:") {
                    pid = line
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or_default()
                        .parse()
                        .unwrap_or_default();
                } else if line.starts_with("PPid:") {
                    ppid = line
                        .split_whitespace()
                        .nth(1)
                        .unwrap_or_default()
                        .parse()
                        .unwrap_or_default();
                } else if line.starts_with("State:") {
                    let state_str = line.split_whitespace().nth(1).unwrap_or_default();
                    state = State::try_from(state_str.chars().next().unwrap_or_default())
                        .unwrap_or_default();
                }
            })
        })?;

        Ok(Process {
            pid,
            ppid,
            name,
            cmd,
            state,
        })
    }
}
