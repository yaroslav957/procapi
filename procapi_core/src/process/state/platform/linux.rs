use crate::process::state::State;
use std::io::{Error, ErrorKind};

impl TryFrom<u8> for State {
    type Error = Error;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        Ok(match c {
            b'R' => Self::Running,
            b'S' => Self::Sleeping,
            b'D' => Self::DiskSleep,
            b'T' => Self::Stopped,
            b't' => Self::TracingStop,
            b'X' => Self::Dead,
            b'Z' => Self::Zombie,
            b'P' => Self::Parked,
            b'I' => Self::Idle,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("[Invalid state character '{c}']"),
                ))
            }
        })
    }
}
