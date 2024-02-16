use crate::description::LevelDescription;
use crate::game_state::GameGridState;
use crate::grid::Grid;
use crate::strategy::SolvingStrategy;
use std::time::{Duration, Instant};
use crate::line::{ColLine, RowLine};

pub struct AiPlayer {
    pub engines: Vec<Box<dyn SolvingStrategy>>,
    current_engine: usize,
    is_active: bool,
    last_update_instant: Instant
}

impl AiPlayer {
    pub fn is_active(&self) -> bool {self.is_active}
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
            if process_lines(level_description, grid, &*self.engines[i]) {//&* is strange
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
            if process_lines(level_description, grid, &*self.engines[self.current_engine]) {
                return;
            }
        }
    }
    pub fn pause_play(&mut self) {
        self.is_active = false
    }
}
///returns true if strategy make any decision, false if not
fn process_lines<GridType: Grid>(level_description: &LevelDescription, grid: &mut GridType, strategy: &dyn SolvingStrategy) -> bool {
    for (row_num, row_description) in level_description.rows.iter().enumerate() {
        if row_description.parts.iter().any(|x| !x.is_completed) {
            let mut line = RowLine::new(grid, row_num);
            if strategy.process_one_line(&row_description, &mut line) {
                return true;
            }
        }
    }

    for (col_num, col_description) in level_description.cols.iter().enumerate() {
        if col_description.parts.iter().any(|x| !x.is_completed) {
            let mut line = ColLine::new(grid, col_num);
            if strategy.process_one_line(&col_description, &mut line) {
                return true;
            }
        }
    }

    false
}