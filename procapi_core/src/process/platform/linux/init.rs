use crate::process::state::State;
use crate::process::thread::Thread;
use crate::process::Process;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

impl TryFrom<u32> for Process {
    type Error = Error;

    fn try_from(pid: u32) -> Result<Self, Self::Error> {
        let proc_dir = Path::new("/proc");
        let pid_dir = proc_dir.join(pid.to_string());
        let comm_dir = pid_dir.join("comm");
        let cmdline_dir = pid_dir.join("cmdline");
        let thread_dir = pid_dir.join("task");
        let status_dir = pid_dir.join("status");

        let mut ppid = u32::default();
        let mut state = State::default();
        let mut threads = Vec::new();

        let name = fs::read_to_string(comm_dir)?.trim().to_string();
        let cmd = fs::read_to_string(cmdline_dir)?.replace('\0', " ");
        let status_content = fs::read_to_string(status_dir)?;
        for line in status_content.lines() {
            match line.split_once(':') {
                Some(("PPid", p)) => ppid = p.trim().parse().unwrap_or_default(),
                Some(("State", s)) => state = State::try_from(s.trim())?,
                _ => {}
            }
        }

        let thread_dir_content = fs::read_dir(thread_dir)?;
        for entry in thread_dir_content {
            let entry = entry?;
            if entry.path().is_dir() {
                if let Some(file_name) = entry.path().file_name().ok_or(ErrorKind::Other)?.to_str()
                {
                    let tid = file_name.parse().unwrap_or_default();
                    threads.push(Thread { tid }); // Later: Thread::try_from(tid)
                }
            }
        }

        Ok(Process {
            pid,
            ppid,
            name,
            cmd,
            state,
            threads,
        })
    }
}
