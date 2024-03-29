use std::collections::{HashMap, VecDeque};
use std::time::Duration;

use crate::geometry::{Dir, DIRECTIONS};
use crate::maze::{Maze, MoveResult};
use crate::visible_area::visible_area;
use crate::utils::tuple_arithmetic::{distance, linear_interpolation};


const MIN_BRIGHTNESS: u8 = 96;
const HINT_USAGE_ALLOWED_INTERVAL: Duration = Duration::from_secs(60);


pub type Camera = (f32, f32);

#[derive(Clone, Copy)]
pub struct VisualInfo {
    pub directly_reachable: bool,
    distance: u32,
    pub brightness: u8,
}

enum State {
    Idle,
    /// Contains duration since last movement.
    MovingToFinish(Duration),
}

pub struct Scene {
    pub maze: Maze,
    pub camera: Camera,
    pub level_id: &'static str,
    pub stage: u32,

    pub visual_info: HashMap<(i32, i32), VisualInfo>,
    state: State,
    time_since_hint_usage: Duration,
}


impl VisualInfo {
    fn update(&mut self, elapsed: Duration) {
        const DIM_PER_MS: f32 = 0.7;
        const DIM_PER_STEP: f32 = 2.7;
        let target = if self.directly_reachable {
            match 255 - (self.distance as f32 * DIM_PER_STEP) as i32 {
                x if x <= i32::from(MIN_BRIGHTNESS) => MIN_BRIGHTNESS,
                x => x as u8,
            }
        } else {
            MIN_BRIGHTNESS
        };

        let max_delta = (elapsed.as_millis() as f32 * DIM_PER_MS) as i32;
        let min = i32::from(self.brightness) - max_delta;
        let max = i32::from(self.brightness) + max_delta;
        self.brightness = match i32::from(target) {
            target if target < min => min,
            target if target > max => max,
            target => target,
        } as u8;
    }
}


impl Scene {
    pub fn new(
        maze: Maze,
        level_id: &'static str,
        stage: u32
    ) -> Scene {
        let player_pos = maze.position();
        let mut result = Scene {
            maze,
            camera: (player_pos.0 as f32, player_pos.1 as f32),
            level_id, stage,
            visual_info: HashMap::new(),
            state: State::Idle,
            time_since_hint_usage: Duration::from_secs(0),
        };
        result.on_position_updated();
        result
    }

    pub fn update(&mut self, elapsed: Duration) {
        self.update_scheduled_movement(elapsed);
        self.update_camera(elapsed);
        for info in self.visual_info.values_mut() {
            info.update(elapsed);
        }
        self.time_since_hint_usage += elapsed;
    }

    fn update_scheduled_movement(&mut self, elapsed: Duration) {
        const MOVEMENT_INTERVAL: Duration = Duration::from_millis(35);
        if let State::MovingToFinish(mut time_since_movement) = self.state {
            time_since_movement += elapsed;
            let mut finish_movement = false;
            while time_since_movement >= MOVEMENT_INTERVAL {
                time_since_movement -= MOVEMENT_INTERVAL;
                match self.maze.try_move_towards_finish() {
                    MoveResult::MovedToUntouched | MoveResult::Finish => {
                        finish_movement = true;
                        break;
                    },
                    _ => {},
                };
            }
            if finish_movement {
                self.state = State::Idle;
            } else {
                self.state = State::MovingToFinish(time_since_movement);
            }
            self.on_position_updated();
        }
    }

    fn update_camera(&mut self, elapsed: Duration) {
        const ACCELERATION_PER_MS: f32 = 0.997;
        let pos = self.maze.position();
        if distance(pos, self.camera) < 0.05 {
            self.camera = (pos.0 as f32, pos.1 as f32);
        } else {
            let ratio = ACCELERATION_PER_MS.powf(elapsed.as_millis() as f32);
            self.camera = linear_interpolation(pos, self.camera, ratio);
        }
    }

    pub fn try_move(&mut self, dir: Dir) -> MoveResult {
        let result = self.maze.try_move(dir);
        if result == MoveResult::MovedToVisited
            || result == MoveResult::MovedToUntouched
        {
            self.on_position_updated();
        }
        result
    }

    pub fn try_move_towards_start(&mut self) -> MoveResult {
        let result = self.maze.try_move_towards_start();
        self.on_position_updated();
        result
    }

    pub fn use_hint(&mut self) {
        if self.time_since_hint_usage >= HINT_USAGE_ALLOWED_INTERVAL {
            self.state = State::MovingToFinish(Duration::from_secs(0));
            self.time_since_hint_usage = Duration::from_secs(0);
        }
    }

    fn on_position_updated(&mut self) {
        self.recalculate_visual_info();
    }

    fn recalculate_visual_info(&mut self) {
        let player_pos = self.maze.position();
        let visible_area = visible_area().shifted_by(player_pos);

        for &cell in visible_area.cells() {
            self.visual_info.entry(cell).or_insert(VisualInfo {
                directly_reachable: false,
                distance: 0,
                brightness: 0,
            });
        }
        self.visual_info.retain(|cell, visual_info| {
            if visible_area.cells().contains(cell) {
                visual_info.directly_reachable = false;
                visual_info.distance = 0;
                true
            } else {
                false
            }
        });

        self.visual_info.get_mut(&player_pos).unwrap().directly_reachable = true;
        let mut queue = VecDeque::from(vec![player_pos]);
        while let Some(pos) = queue.pop_front() {
            let distance = self.visual_info[&pos].distance;
            for &dir in &DIRECTIONS {
                let to = pos + dir;
                if self.maze.current_layer().passable(pos, dir)
                    && visible_area.cells().contains(&to)
                    && !self.visual_info[&to].directly_reachable
                {
                    let info = &mut self.visual_info.get_mut(&to).unwrap();
                    info.directly_reachable = true;
                    info.distance = distance + 1;
                    queue.push_back(to);
                }
            }
        }
    }
}
