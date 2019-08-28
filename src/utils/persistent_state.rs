use std::collections::HashMap;
use std::sync::Mutex;
use crate::observers::level_completion_observer;

pub struct Progress {
    pub completed_stages: HashMap<&'static str, u64>,
}

pub struct PersistentState {
    pub progress: Progress,
}

impl Progress {
    pub fn complete_level(&mut self, level: &'static str) {
        *self.completed_stages.entry(level).or_insert(0) += 1;
    }

    pub fn completed_stages(&self, level: &'static str) -> u64 {
        self.completed_stages.get(level).cloned().unwrap_or(0)
    }
}

impl PersistentState {
    fn initialize() -> Self {
        level_completion_observer().lock().unwrap().observe(|event| {
            let mut state = get_persistent_state().lock().unwrap();
            state.progress.complete_level(event.level);
        });
        Self {
            progress: Progress {
                completed_stages: HashMap::new()
            }
        }
    }
}

pub fn get_persistent_state() -> &'static Mutex<PersistentState> {
    lazy_static! {
        static ref PERSISTENT_STATE: Mutex<PersistentState> = {
            Mutex::new(PersistentState::initialize())
        };
    }
    &PERSISTENT_STATE
}
