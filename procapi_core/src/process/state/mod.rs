mod platform;

use std::io::Error;

#[derive(Debug, Clone, Default)]
pub enum State {
    /*
    `Embryo` = Initialized (Windows), Idle (Linux)
    `Sleeping` = Ready (Windows), Sleeping (Linux), Stopped (macOS), Parked (Linux) <- CPU Sleep
    `Waiting` = Transition (Windows), DiskSleep (Linux) <- IO Sleep + Wait (Windows), Stopped (Linux), Waiting (macOS) <- Resources Sleep + TracingStop (Linux)
    `Running` = Standby (Windows) + Running (Windows), Running (Linux), Running (macOS)
    `Dead` = Terminated (Windows), Dead (Linux) + Zombie (Linux), Halted (macOS)
    `Uninterruptible` = Protected win-Processes, WaitingUninterruptible (macOS)
     */
    #[default]
    Embryo,
    Sleeping,
    Waiting,
    Running,
    Dead,
    Uninterruptible,
}

impl TryFrom<&str> for State {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(State::try_from(s.as_bytes()[0]).unwrap_or_default())
    }
}
