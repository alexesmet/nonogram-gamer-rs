use crate::description::LevelDescription;
use crate::game_state::GameGridState;
use crate::grid::Grid;
use crate::strategy::SolvingStrategy;
use std::time::{Duration, Instant};

pub struct AiPlayer {
    pub engines: Vec<Box<dyn SolvingStrategy>>,
    current_engine: usize,
    pub is_active: bool,
    last_update_instant: Instant
}

impl AiPlayer {
    pub fn new() -> Self {
        let engines: Vec<Box<dyn SolvingStrategy>> = Vec::new();
        Self {
            engines,
            is_active: false,
            current_engine: 0,
            last_update_instant: Instant::now()
        }
    }
    pub fn play_single_turn_emergency<GridType: Grid>(&self, level_description: &LevelDescription, grid: &mut GridType) {
        for i in 0..self.engines.len() {
            if self.engines[i].play_one_turn(level_description, grid) {
                return;
            }
        }
    }

    pub fn try_perform_turn<GridType: Grid>(&mut self, level_description: &LevelDescription, grid: &mut GridType) {
        if self.is_active && self.last_update_instant.elapsed().as_secs_f32() > 1.0 {
            self.last_update_instant = Instant::now();
            self.play_single_turn_with_engines_order_memory(level_description, grid);
        }
    }
    pub fn start_play(&mut self) {
        self.last_update_instant = Instant::now();
        self.is_active = true;
    }
    pub fn restart_clock(&mut self) {
        self.last_update_instant = Instant::now();
    }
    fn play_single_turn_with_engines_order_memory<GridType: Grid>(&mut self, level_description: &LevelDescription, grid: &mut GridType) {
        for i in 0..self.engines.len() {
            self.current_engine = (self.current_engine + i) % self.engines.len();
            if self.engines[self.current_engine].play_one_turn(level_description, grid) {
                return;
            }
        }
    }
    pub fn pause_play(&mut self) {
        self.is_active = false
    }
}