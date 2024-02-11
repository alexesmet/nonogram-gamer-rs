use crate::description::LevelDescription;
use crate::grid::Grid;

pub trait PlayEngine {
    fn play_one_turn(&self, level_description: &LevelDescription, grid: &mut dyn Grid) -> bool;
}