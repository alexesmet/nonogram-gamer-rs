use crate::description::{LevelDescription, LineDescription};
use crate::game_state::CellState;
use crate::grid::Grid;
use crate::line::Line;
use crate::strategy::SolvingStrategy;

pub struct SimpleStrategy {

}
impl SolvingStrategy for SimpleStrategy {

    fn process_one_line(&self, line_description: &LineDescription, line: &mut dyn Line) -> bool {

        let begin_offset = (0..line.len()).take_while(|i| line.get(*i) == CellState::Crossed).count();
        let end_offset = (0..line.len()).rev().take_while(|i| line.get(*i) == CellState::Crossed).count();

        let total_filled_count = line_description.parts.iter().map(|x| x.elements_count).sum::<usize>();
        let total_crossed_count = line_description.parts.len() - 1;
        let current_emplace_target_count = line.len() - begin_offset - end_offset;

        if total_filled_count + total_crossed_count == current_emplace_target_count {
            let mut index = begin_offset;
            for element_index in 0..line_description.parts.len() - 1 {
                for i in 0..line_description.parts[element_index].elements_count {
                    line.set(index, CellState::Filled);
                    index += 1;
                }
                line.set(index, CellState::Crossed);
                index += 1;
            }
            for i in 0..line_description.parts[line_description.parts.len() - 1].elements_count {
                line.set(index, CellState::Filled);
                index += 1;
            }
            return true;
        }

        false
    }
}
