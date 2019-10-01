use std::sync::Mutex;

pub trait ObservableEvent: Clone {}

pub struct Observer<Event> {
    observers: Vec<fn(Event)>
}

impl<Event> Observer<Event> where Event: ObservableEvent {
    pub fn new() -> Self {
        Self {
            observers: Vec::new()
        }
    }

    pub fn notify(&self, event: Event) {
        for observer in &self.observers {
            observer(event.clone());
        }
    }

    pub fn observe(&mut self, f: fn(Event)) {
        self.observers.push(f);
    }
}

#[derive(Clone)]
pub struct LevelCompleted {
    pub level: &'static str,
    pub stage: u32,
}
impl ObservableEvent for LevelCompleted {}

pub fn level_completion_observer() -> &'static Mutex<Observer<LevelCompleted>> {
    lazy_static! {
        static ref LEVEL_COMPLETION_OBSERVER: Mutex<Observer<LevelCompleted>> = {
            Mutex::new(Observer::new())
        };
    }
    &LEVEL_COMPLETION_OBSERVER
}
