use std::io::Error;
use procfs::process::Process as SysProcess;
use crate::process::{Process, State};

pub fn get_processes() -> Vec<Process> {
    procfs::process::all_processes()
        .ok()
        .expect("[ProcessesIter error]")
        .flatten()
        .filter_map(|p| p.stat().ok())
        .filter_map(|stat| Process::try_from(stat.pid).ok())
        .collect::<Vec<Process>>()
}

impl TryFrom<i32> for Process {
    type Error = Error;

    fn try_from(pid: i32) -> Result<Self, Self::Error> {
        let proc = SysProcess::new(pid)
            .map_err(|_| Error::last_os_error())?;
        match proc.stat() {
            Ok(stat) => {
                let cmd = proc.cmdline()
                    .unwrap_or_else(|_| {
                    if let Ok(exe) = proc.exe() {
                        vec![exe.display()
                            .to_string()]
                    } else {
                        vec![String::from("?")]
                    }
                });

                Ok(Self {
                    ids: [stat.pid as u32, stat.ppid as u32],
                    state: State::try_from(stat.state)
                        .map_err(|_| Error::last_os_error())?,
                    cmd: cmd.join(" "),
                    name: cmd.first()
                        .cloned()
                        .unwrap_or_default(),
                })
            }
            Err(_) => Err(Error::last_os_error()),
        }
    }
}