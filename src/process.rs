use std::process::Command;
use std::str::FromStr;

pub struct Process {
    ids: [u32; 2],
    name: String,
    cmd: String,
    state: String,
    //usage
}

#[cfg(unix)]
pub fn processes_info() -> Result<Vec<Process>, std::io::Error> {
    let output = Command::new("ps")
        .arg("-eo")
        .arg("pid,ppid,comm,state,command")
        .output()?;
    let output_str = String::from_utf8_lossy(&output.stdout);
    let processes = output_str.lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let pid = u32::from_str(parts[0]).unwrap_or(0);
                let ppid = u32::from_str(parts[1]).unwrap_or(0);
                let name = parts[2].to_string();
                let state = parts[3].to_string();
                let cmd = parts[4..].join(" ");

                Some(Process {
                    name,
                    ids: [pid, ppid],
                    cmd,
                    state,
                })
            } else {
                None
            }
        })
        .collect();

    Ok(processes)
}