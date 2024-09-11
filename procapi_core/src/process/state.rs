use std::io::{Error, ErrorKind};

#[derive(Debug, Clone, Default)]
pub enum State {
    Idle,
    Runnable,
    #[default]
    Sleeping,
    Stopped,
    UninterruptibleWait,
    Dead,
}

impl TryFrom<&str> for State {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(State::try_from(s.as_bytes()[0] as char).unwrap_or_default())
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
            'U' | 'D' => Self::UninterruptibleWait, // AKA Unix uninterruptible sleep
            'Z' => Self::Dead,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("[Invalid state character '{c}']"),
                ))
            }
        })
    }
}
