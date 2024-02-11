use crate::description::{LevelDescription, LineDescription};
use crate::grid::Grid;
use crate::line::Line;

pub mod simple;

pub trait SolvingStrategy {
    fn process_one_line(&self, level_description: &LineDescription, line: &mut dyn Line) -> bool;
}
