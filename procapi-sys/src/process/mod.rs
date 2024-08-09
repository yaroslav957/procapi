#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "freebsd")]
mod bsd;
#[cfg(target_os = "freebsd")]
pub use bsd::*;

use std::io::{Error, ErrorKind};

#[derive(Debug, Clone)]
pub struct Process {
    pub ids: [u32; 2],
    pub name: String,
    pub cmd: String,
    pub state: State,
    //usage
}

#[derive(Debug, Clone)]
pub enum State {
    Idle,
    Runnable,
    Sleeping,
    Stopped,
    UninterruptibleWait,
    Dead,
}

impl TryFrom<&str> for State {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(State::try_from(s.as_bytes()[0] as char)
            .unwrap_or(State::Dead))
    }
}

impl TryFrom<char> for State {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            'I' => Self::Idle,
            'R' => Self::Runnable,
            'S' => Self::Sleeping,
            'T' => Self::Stopped,
            'U' => Self::UninterruptibleWait,
            'Z' => Self::Dead,
            _ => return Err(
                Error::new(ErrorKind::InvalidInput, "Invalid state")
            )
        })
    }
}