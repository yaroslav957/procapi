use std::fs;
use crate::process::{Process, State};
use std::io::{self, BufRead};
use std::io::Error;
use std::path::Path;

//фулл передапать хуйню

const PROC_DIR: &'static str = "/proc";

const COMM_PATH: &'static str = "comm";
const STATUS_PATH: &'static str = "status";
const CMD_PATH: &'static str = "cmdline";

pub fn get_processes() -> Vec<Process> {
    let proc_dir = Path::new(PROC_DIR);
    let mut pids = Vec::new();

    if proc_dir.exists() {
        for entry in fs::read_dir(proc_dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                if let Ok(pid) = entry.file_name().into_string().unwrap().parse::<u32>() {
                    pids.push(pid);
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

        let status_path = pid_dir.join("status");
        let comm_path = pid_dir.join("comm");
        let cmdline_path = pid_dir.join("cmdline");

        let mut ids = [0; 2];
        let mut name = String::new();
        let mut cmd = String::new();
        let mut state = State::Runnable;

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

        if let Ok(comm_file) = fs::File::open(&comm_path) {
            let mut reader = io::BufReader::new(comm_file);
            reader.read_line(&mut name).unwrap();
            name = name.trim().to_string();
        }

        if let Ok(cmdline_file) = fs::File::open(&cmdline_path) {
            let mut reader = io::BufReader::new(cmdline_file);
            reader.read_line(&mut cmd).unwrap();
            cmd = cmd.trim().to_string();
        }

        Ok(Process {
            ids,
            name,
            cmd,
            state,
        })
    }
}
