use std::io::{Error, ErrorKind};

mod platform;
pub use platform::*;

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
                Error::new(ErrorKind::InvalidInput, "[Invalid parse state]")
            )
        })
    }
}