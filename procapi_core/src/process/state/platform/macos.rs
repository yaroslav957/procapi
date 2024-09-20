use crate::process::state::State;

impl State {
    pub(crate) fn from_pth_info(
        run_state: i32,
        sleep_time: i32,
    ) -> State {
        match run_state {
            1 => State::Running,
            2 => State::Sleeping,
            3 => {
                if sleep_time > 20 {
                    State::Waiting
                } else {
                    State::Embryo
                }
            }
            4 => State::Uninterruptible,
            5 => State::Dead,
            _ => unreachable!("[Unknown pth_run_state]"),
        }
    }
}
