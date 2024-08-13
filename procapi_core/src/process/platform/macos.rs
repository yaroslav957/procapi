use libc::{c_void, size_t};
use libproc::{
    libproc::proc_pid,
    libproc::proc_pid::{pidpath, ListThreads},
    proc_pid::listpidinfo,
    proc_pid::pidinfo,
    processes::{pids_by_type, ProcFilter},
    task_info::TaskAllInfo,
    thread_info::ThreadInfo,
};
use std::{
    io::{Error, ErrorKind},
    ptr::null_mut,
};

use crate::process::{Process, State};

pub fn get_processes() -> Vec<Process> {
    pids_by_type(ProcFilter::All)
        .unwrap_or_default()
        .iter()
        .filter_map(|&pid| Process::try_from(pid as i32).ok())
        .collect::<Vec<Process>>()
}

impl TryFrom<i32> for Process {
    type Error = Error;

    fn try_from(pid: i32) -> Result<Self, Self::Error> {
        if let Ok(info) = proc_pid::pidinfo::<TaskAllInfo>(pid, 0) {
            let pth_state = listpidinfo::<ListThreads>(pid, info.ptinfo.pti_threadnum as usize)
                .unwrap_or_default()
                .iter()
                .filter_map(|&t| pidinfo::<ThreadInfo>(pid, t).ok())
                .map(|t| match t.pth_run_state {
                    1 => 1, // TH_STATE_RUNNING
                    2 => 5, // TH_STATE_STOPPED
                    3 => {
                        // TH_STATE_WAITING
                        if t.pth_sleep_time > 20 {
                            4
                        } else {
                            3
                        }
                    }
                    4 => 2, // TH_STATE_UNINTERRUPTIBLE
                    5 => 6, // TH_STATE_HALTED
                    _ => 7,
                })
                .min()
                .unwrap_or(7);

            dbg!(info.pbsd);

            Ok(Process {
                ids: [pid as u32, info.pbsd.pbi_ppid],
                name: proc_pid::name(pid).unwrap_or_else(|_| pidpath(pid).unwrap_or_default()),
                state: State::try_from(pth_state)?,
                cmd: get_cmdline(pid as u32)
                    .unwrap_or_else(|| vec![pidpath(pid).unwrap_or_default()])
                    .join(" "),
            })
        } else {
            Err(Error::last_os_error())
        }
    }
}

impl TryFrom<u32> for State {
    type Error = Error;

    fn try_from(pth_state: u32) -> Result<Self, Self::Error> {
        Ok(match pth_state {
            1 => State::Runnable,
            2 => State::UninterruptibleWait,
            3 => State::Sleeping,
            4 => State::Idle,
            5 => State::Stopped,
            6 => State::Dead,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("unknown pth state: {pth_state}"),
                ))
            }
        })
    }
}

fn get_cmdline(pid: u32) -> Option<Vec<String>> {
    let mut mib: [libc::c_int; 3] = [
        libc::CTL_KERN,
        libc::KERN_PROCARGS2,
        pid.try_into().unwrap(),
    ];
    let mut size: size_t = get_argmax();
    let process_args: *mut u8 =
        unsafe { std::mem::transmute(libc::calloc(size, std::mem::size_of::<size_t>())) };
    let mut res = Vec::<String>::new();

    assert_ne!(
        process_args,
        null_mut(),
        "process args buffer allocation failed"
    );

    unsafe {
        if libc::sysctl(
            mib.as_mut_ptr(),
            3,
            process_args as *mut c_void,
            &mut size as *mut usize,
            core::ptr::null_mut(),
            0,
        ) == -1
        {
            libc::free(process_args as *mut c_void);
            return None;
        }

        let mut arg_start: *mut u8 = null_mut();
        let mut ch_ptr: *mut u8 = process_args.add(std::mem::size_of::<libc::c_int>());

        for _ in 0..size {
            if *ch_ptr == b'\0' {
                if !arg_start.is_null() && arg_start != ch_ptr {
                    res.push(get_str_unchecked(arg_start, ch_ptr));
                }

                arg_start = ch_ptr.add(1);
            }

            ch_ptr = ch_ptr.add(1);
        }
    }

    return Some(res);
}

/// Get buffer size reserved for arguments string
fn get_argmax() -> size_t {
    let mut sys_max_args = 0i32;
    let mut size = std::mem::size_of::<libc::c_int>();
    let mut mib: [libc::c_int; 2] = [libc::CTL_KERN, libc::KERN_ARGMAX];

    unsafe {
        libc::sysctl(
            mib.as_mut_ptr(),
            2,
            (&mut sys_max_args) as *mut i32 as *mut c_void,
            &mut size,
            null_mut(),
            0,
        );
    }

    sys_max_args as size_t
}

/// Used for extracting data retrieved from kernel, which is guaranteed
/// to be a valid string
unsafe fn get_str_unchecked(
    start: *mut u8,
    end: *mut u8,
) -> String {
    let len = end as usize - start as usize;
    let bytes = Vec::from_raw_parts(start, len, len);
    let s = String::from_utf8_unchecked(bytes.clone());
    std::mem::forget(bytes);
    s
}
