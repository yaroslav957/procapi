mod init;

use crate::process::Process;
use std::fs;
use std::io::Error;

pub fn get_processes() -> Result<Vec<Process>, Error> {
    let mut processes = Vec::new();
    let entries = fs::read_dir("/proc")?;

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };

        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let Ok(pid) = entry
            .file_name()
            .into_string()
            .ok()
            .ok_or(Error::last_os_error())?
            .parse::<u32>()
        else {
            continue;
        };

        if let Ok(proc) = Process::try_from(pid) {
            processes.push(proc);
        }
    }

    Ok(processes)
}
