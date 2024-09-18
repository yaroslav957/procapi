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

use crate::process::{Process, State, Thread};

pub fn get_processes() -> Result<Vec<Process>, Error> {
    Ok(pids_by_type(ProcFilter::All)
        .unwrap_or_default()
        .iter()
        .filter_map(|&pid| Process::try_from(pid as i32).ok())
        .collect::<Vec<Process>>())
}

impl TryFrom<i32> for Process {
    type Error = Error;

    fn try_from(pid: i32) -> Result<Self, Self::Error> {
        if let Ok(info) = proc_pid::pidinfo::<TaskAllInfo>(pid, 0) {
            let threads = listpidinfo::<ListThreads>(pid, info.ptinfo.pti_threadnum as usize)
                .unwrap_or_default();
            let pth_state = threads
                .iter()
                .filter_map(|&t| pidinfo::<ThreadInfo>(pid, t).ok())
                .map(|t| match t.pth_run_state {
                    1 => State::Running,
                    2 => State::Sleeping,
                    3 => {
                        if t.pth_sleep_time > 20 {
                            State::Waiting
                        } else {
                            State::Embryo
                        }
                    }
                    4 => State::Uninterruptible,
                    5 => State::Dead,
                    _ => unreachable!("unknown pth_run_state"),
                })
                .min()
                .unwrap_or_default();

            Ok(Process {
                pid: pid as u32,
                ppid: info.pbsd.pbi_ppid,
                name: proc_pid::name(pid).unwrap_or_else(|_| pidpath(pid).unwrap_or_default()),
                state: pth_state,
                cmd: Process::get_cmdline(pid as u32)
                    .unwrap_or_else(|| vec![pidpath(pid).unwrap_or_default()])
                    .join(" "),
                threads: threads
                    .iter()
                    .map(|&tid| Thread { tid })
                    .collect::<Vec<Thread>>(),
            })
        } else {
            Err(Error::last_os_error())
        }
    }
}

impl Process {
    fn get_cmdline(pid: u32) -> Option<Vec<String>> {
        let mut mib = [
            libc::CTL_KERN,
            libc::KERN_PROCARGS2,
            pid.try_into().unwrap(),
        ];
        let mut size = get_argmax();
        let mut process_args = Vec::<u8>::with_capacity(size);
        let mut res = Vec::<String>::new();

        unsafe {
            if libc::sysctl(
                mib.as_mut_ptr(),
                mib.len() as libc::c_uint,
                process_args.as_mut_ptr() as *mut c_void,
                &mut size as *mut usize,
                core::ptr::null_mut(),
                0,
            ) == -1
            {
                return None;
            }

            let mut arg_num: libc::c_int = 0;
            libc::memcpy(
                (&mut arg_num) as *mut libc::c_int as *mut c_void,
                process_args.as_ptr() as *const c_void,
                std::mem::size_of::<libc::c_int>(),
            );

            let mut arg_start: *mut u8 = null_mut();
            let mut ch_ptr: *mut u8 = process_args
                .as_mut_ptr()
                .add(std::mem::size_of::<libc::c_int>());

            for _ in 0..size {
                if arg_num == 0 {
                    break;
                }

                if *ch_ptr == b'\0' {
                    if !arg_start.is_null() && arg_start != ch_ptr {
                        res.push(get_str_checked(arg_start, ch_ptr));
                    }

                    arg_num -= 1;
                    arg_start = ch_ptr.add(1);
                }

                ch_ptr = ch_ptr.add(1);
            }
        }

        Some(res)
    }
}

/// Get buffer size reserved for arguments string
fn get_argmax() -> size_t {
    let mut sys_max_args = 0i32;
    let mut size = std::mem::size_of::<libc::c_int>();
    let mut mib: [libc::c_int; 2] = [libc::CTL_KERN, libc::KERN_ARGMAX];

    unsafe {
        libc::sysctl(
            mib.as_mut_ptr(),
            mib.len() as libc::c_uint,
            (&mut sys_max_args) as *mut i32 as *mut c_void,
            &mut size,
            null_mut(),
            0,
        );
    }

    sys_max_args as size_t
}

#[inline(always)]
unsafe fn get_str_checked(
    start: *mut u8,
    end: *mut u8,
) -> String {
    let len = end as usize - start as usize;
    let bytes = std::slice::from_raw_parts(start, len);
    let s = std::str::from_utf8(bytes);
    s.unwrap_or("").to_owned()
}
