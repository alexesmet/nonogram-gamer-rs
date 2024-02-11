use crate::description::LevelDescription;
use crate::game_state::CellState;
use crate::grid::Grid;
use crate::play_engine::PlayEngine;

pub struct SimpleEngine {

}
impl PlayEngine for SimpleEngine {
    fn play_one_turn(&self, level_description: &LevelDescription, grid: &mut dyn Grid) -> bool {
        for row_description in level_description.rows.iter().enumerate() {
            if !row_description.1.iter().all(|x| x.1) {
                let mut begin_offset = 0;
                let mut end_offset = 0;

                for i in 0..grid.width() {
                    if grid.get(i, row_description.0) == CellState::Crossed {
                        begin_offset += 1;
                    }
                    else {
                        break;
                    }
                }

                for i in (0..grid.width()).rev() {
                    if grid.get(i, row_description.0) == CellState::Crossed {
                        end_offset += 1;
                    }
                    else {
                        break;
                    }
                }

                let total_filled_count = row_description.1.iter().map(|x| x.0).sum::<usize>();
                let total_crossed_count = row_description.1.len() - 1;
                let current_emplace_target_count = grid.width() - begin_offset - end_offset;

                if total_filled_count + total_crossed_count == current_emplace_target_count {
                    let mut col_index = begin_offset;
                    for row_element_index in 0..row_description.1.len() - 1 {
                        for i in 0..row_description.1[row_element_index].0 {
                            grid.set(col_index, row_description.0, CellState::Filled);
                            col_index += 1;
                        }
                        grid.set(col_index, row_description.0, CellState::Crossed);
                        col_index += 1;
                    }
                    for i in 0..row_description.1[row_description.1.len() - 1].0 {
                        grid.set(col_index, row_description.0, CellState::Filled);
                        col_index += 1;
                    }
                    return true;
                }
            }
        }

        for col_description in level_description.cols.iter().enumerate() {
            if !col_description.1.iter().all(|x| x.1) {

                let mut begin_offset = 0;
                let mut end_offset = 0;

                for i in 0..grid.height() {
                    if grid.get(i, col_description.0) == CellState::Crossed {
                        begin_offset += 1;
                    }
                    else {
                        break;
                    }
                }

                for i in (0..grid.height()).rev() {
                    if grid.get(i, col_description.0) == CellState::Crossed {
                        end_offset += 1;
                    }
                    else {
                        break;
                    }
                }

                let total_filled_count = col_description.1.iter().map(|x| x.0).sum::<usize>();
                let total_crossed_count = col_description.1.len() - 1;
                let current_emplace_target_count = grid.width() - begin_offset - end_offset;

                if total_filled_count + total_crossed_count == current_emplace_target_count {
                    let mut row_index = begin_offset;
                    for col_element_index in 0..col_description.1.len() - 1 {
                        for i in 0..col_description.1[col_element_index].0 {
                            grid.set(col_description.0, row_index, CellState::Filled);
                            row_index += 1;
                        }
                        grid.set(col_description.0, row_index, CellState::Crossed);
                        row_index += 1;
                    }
                    for i in 0..col_description.1[col_description.1.len() - 1].0 {
                        grid.set(col_description.0, row_index, CellState::Filled);
                        row_index += 1;
                    }
                    return true;
                }
            }
        }

        false
    }
}