mod platform;

#[derive(Debug, Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub enum State {
    #[default]
    /// Standby (Windows) + Running (Windows), Running (Linux), Running (macOS)
    Running,
    /// Transition (Windows), DiskSleep (Linux) <- IO Sleep + Wait (Windows), Stopped (Linux), Waiting (macOS) <- Resources Sleep + TracingStop (Linux)
    Waiting,
    /// Protected win-Processes, Uninterruptible (macOS)
    Uninterruptible,
    /// Ready (Windows), Sleeping (Linux), Stopped (macOS), Parked (Linux) <- CPU Sleep
    Sleeping,
    /// Initialized (Windows), Idle (Linux)
    Embryo,
    /// Terminated (Windows), Dead (Linux) + Zombie (Linux), Halted (macOS)
    Dead,
}
