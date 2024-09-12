mod platform;

use std::io::Error;

#[derive(Debug, Clone, Default)]
pub enum State {
    Running,
    #[default]
    Sleeping,
    DiskSleep,
    Stopped,
    TracingStop,
    Dead,
    Zombie,
    Parked,
    Idle,
}

impl TryFrom<&str> for State {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(State::try_from(s.as_bytes()[0]).unwrap_or_default())
    }
}
