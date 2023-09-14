use std::ops::{Deref, DerefMut};

use crate::utils::doryen::Console;

pub enum ConsoleState {
    Available,
    Busy,
}

impl std::fmt::Debug for ConsoleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ConsoleState::Available => write!(f, "<available>"),
            ConsoleState::Busy => write!(f, "<busy>"),
        }
    }
}

pub struct RawConsole(pub(crate) Option<Console>);

impl RawConsole {
    pub fn new() -> Self {
        RawConsole::default()
    }

    pub fn state(&self) -> ConsoleState {
        if self.0.is_some() {
            ConsoleState::Available
        } else {
            ConsoleState::Busy
        }
    }
}

impl Default for RawConsole {
    fn default() -> Self {
        RawConsole(Some(Console::new(0, 0)))
    }
}

impl std::fmt::Debug for RawConsole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("RawConsole")
            .field(&self.state())
            .finish()
    }
}

impl Deref for RawConsole {
    type Target = Console;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0
            .as_ref()
            .expect("Raw console should be interacted with in `setup`, `update` or `render` systems")
    }
}

impl DerefMut for RawConsole {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
            .as_mut()
            .expect("Raw console should be interacted with in `setup`, `update` or `render` systems")
    }
}