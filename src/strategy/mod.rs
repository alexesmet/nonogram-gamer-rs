use crate::description::LevelDescription;
use crate::grid::Grid;

pub mod simple;

pub trait SolvingStrategy {
    // TODO: chainge the interface so that it only works with a single LINE ans its description
    fn play_one_turn(&self, level_description: &LevelDescription, grid: &mut dyn Grid) -> bool;
}
