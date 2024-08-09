use std::io::Error;
use procfs::process::Process as SysProcess;

use crate::process::{Process, State};

impl TryFrom<i32> for Process {
    type Error = Error;

    fn try_from(pid: i32) -> Result<Self, Self::Error> {
        let proc = SysProcess::new(pid).unwrap();
        if let Ok(stat) = proc.stat() {
            let cmd = proc.cmdline().unwrap_or_else(|_| {
                if let Ok(exe) = proc.exe() {
                    vec![exe.display().to_string()]
                } else {
                    vec![String::from("?")]
                }
            });

            return Ok(Self {
                ids: [stat.pid as u32, stat.ppid as u32],
                state: State::try_from(stat.state)?,
                cmd: cmd.join(" "),
                name: if cmd.len() > 0 { cmd[0].clone() } else { "".into() },
            });
        }

        Err(Error::last_os_error())
    }
}

pub fn get_processes() -> Vec<Process> {
    procfs::process::all_processes()
        .unwrap()
        .flatten()
        .filter_map(|p| p.stat().ok())
        .filter_map(|stat| Process::try_from(stat.pid).ok())
        .collect::<Vec<Process>>()
}
