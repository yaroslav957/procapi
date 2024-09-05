mod platform;
pub mod state;

use crate::process::state::State;
pub use platform::*;

#[derive(Debug, Clone)]
pub struct Process {
    pub ids: [u32; 2], // оставить так как есть, но мб можно просто заалиасить? Надо нано спросить кринж чи не
    pub name: String,  // Структуркой
    pub cmd: String,   // Структуркой
    pub state: State,
    // usage
}
