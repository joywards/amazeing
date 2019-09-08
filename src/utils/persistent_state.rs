use std::collections::HashMap;
use std::sync::Mutex;
use std::path::{Path, PathBuf};
use crate::observers::level_completion_observer;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Progress {
    pub completed_stages: HashMap<String, u64>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PersistentState {
    pub progress: Progress,
}

impl Progress {
    pub fn complete_level(&mut self, level: &'static str) {
        *self.completed_stages.entry(level.to_string()).or_insert(0) += 1;
    }

    pub fn completed_stages(&self, level: &'static str) -> u64 {
        self.completed_stages.get(level).copied().unwrap_or(0)
    }
}

impl PersistentState {
    fn initialize() -> Self {
        level_completion_observer().lock().unwrap().observe(|event| {
            let mut state = get_persistent_state().lock().unwrap();
            state.progress.complete_level(event.level);
            state.flush();
        });

        let path = Self::data_path();
        if path.exists() {
            Self::load_state(&path)
        } else {
            Self::empty()
        }
    }

    fn load_state(path: &Path) -> Self {
        let f = std::fs::File::open(path).expect(
            "Failed opening data file"
        );
        ron::de::from_reader(f).unwrap()
    }

    fn empty() -> Self {
        Self {
            progress: Progress {
                completed_stages: HashMap::new()
            }
        }
    }

    fn data_path() -> PathBuf {
        let mut path = dirs::data_dir().unwrap();
        path.push("amazeing/data.ron");
        path
    }

    fn flush(&self) {
        use std::io::Write;
        use std::fs::{File, create_dir_all};

        let path = Self::data_path();
        create_dir_all(path.parent().unwrap()).unwrap();
        let mut f = File::create(Self::data_path()).expect(
            "Failed opening data file"
        );
        f.write_all(ron::ser::to_string_pretty(
            self,
            ron::ser::PrettyConfig::default()
        ).unwrap().as_bytes()).unwrap();
    }
}


lazy_static! {
    static ref PERSISTENT_STATE: Mutex<PersistentState> = {
        Mutex::new(PersistentState::initialize())
    };
}

pub fn get_persistent_state() -> &'static Mutex<PersistentState> {
    &PERSISTENT_STATE
}
