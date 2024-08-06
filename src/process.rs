use std::process::Command;
use std::str::FromStr;
use itertools::Itertools;

pub struct Process {
    ids: [u32; 2],
    name: String,
    state: String,
    cmd: String,
    //usage
}

impl Process {
    pub fn new(ids: [u32; 2], name: String, state: String, cmd: String) -> Self {
        Self {
            ids,
            name,
            cmd,
            state,
        }
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
                Some(Process::new([pid, ppid], name.to_string(), state.to_string(), cmd))
            } else { None }
        })
        .collect();

    Ok(processes)
}