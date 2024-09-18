use crate::process::state::State;
use std::io::{Error, ErrorKind};

impl TryFrom<u8> for State {
    type Error = Error;

    fn try_from(c: u8) -> Result<Self, Self::Error> {
        // For more information see:
        // https://github.com/torvalds/linux/blob/77f587896757708780a7e8792efe62939f25a5ab/fs/proc/array.c#L126
        Ok(match c {
            b'R' => Self::Running,
            b'S' | b'P' => Self::Sleeping,
            b'D' | b'T' | b't' => Self::Waiting,
            b'X' | b'Z' => Self::Dead,
            b'I' => Self::Embryo,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("[Invalid/Unimplemented state character '{}']", c as char),
                ))
            }
        })
    }
}
