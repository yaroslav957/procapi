use std::fs;
use crate::process::{Process, State};
use std::io::{self, BufRead};
use std::io::Error;
use std::path::Path;

pub fn get_processes() -> Vec<Process> {
    let mut pids = Vec::new();

    if let Ok(entries) = fs::read_dir("/proc") {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    if let Ok(pid) = entry.file_name()
                        .into_string()
                        .unwrap_or_default()
                        .parse::<u32>() {
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

        let status_path = pid_dir.join("stat");
        let comm_path = pid_dir.join("comm");
        let cmdline_path = pid_dir.join("cmdline");

        let mut ids = [0; 2];
        let name = fs::read_to_string(&comm_path)?
            .trim()
            .to_string();
        let cmd = fs::read_to_string(&cmdline_path)?
            .trim()
            .to_string()
            .replace('\0', " ");
        let mut state = State::Runnable;
        
        // ОСТАЛОСЬ ФИКСАНУТЬ ЭТО УЕБИЩЕ
        if let Ok(status_file) = fs::File::open(&status_path) {
            let reader = io::BufReader::new(status_file);
            for line in reader.lines() {
                let line = line.unwrap();
                if line.starts_with("Pid:") {
                    ids[0] = line.split_whitespace().nth(1).unwrap().parse().unwrap();
                } else if line.starts_with("PPid:") {
                    ids[1] = line.split_whitespace().nth(1).unwrap().parse().unwrap();
                } else if line.starts_with("State:") {
                    let state_str = line.split_whitespace().nth(1).unwrap();
                    state = State::try_from(state_str.chars().next().unwrap()).unwrap();
                }
            }
        }
        // ОСТАЛОСЬ ФИКСАНУТЬ ЭТО УЕБИЩЕ ^^^
        
        Ok(Process {
            ids,
            name,
            cmd,
            state,
        })
    }
}
