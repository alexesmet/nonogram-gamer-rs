use ggez::input::gamepad::gilrs::GilrsBuilder;

use crate::description::LevelDescription;


#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Filled,
    Crossed
}

pub struct GameState {
    lvl_desc: LevelDescription,
    grid: Vec<Vec<CellState>>, // в ГРИДЕ хранятся СТРОКИ блять
    height: usize,
    width: usize
}

impl GameState {
    pub fn new(lvl_desc: LevelDescription) -> Self {
    let width = lvl_desc.cols.len();
    let height = lvl_desc.rows.len();
        let grid = vec![vec![CellState::Empty; width]; height];
        Self { lvl_desc, grid, height, width }
    }
    pub fn get(&self, col: usize, row: usize) -> CellState {
        self.grid[row][col]
    }
    pub fn set(&mut self, col: usize, row: usize, val: CellState) {
        self.set_no_update(col, row, val);
        if val == CellState::Filled {
            self.update(col, row);
        }
    }
    fn set_no_update(&mut self, col: usize, row: usize, val: CellState) {
        self.grid[row][col] = val;
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn lvl_desc(&self) -> &LevelDescription {
        &(self.lvl_desc)
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, CellState)> + '_ {
        self.grid.iter().enumerate().flat_map(|(row_id, row)| row.iter().enumerate().map(move |(col_id, cell)| (col_id, row_id, *cell )))
    }
    pub fn row_to_line(&self, row: usize) -> Vec<CellState> {
        self.grid[row].clone()
    }
    pub fn col_to_line(&self, col: usize) -> Vec<CellState> {
        (0..self.height).into_iter().map(|i| self.get(col, i)).collect()
    }
    fn update(&mut self, col: usize, row: usize) {
        if self.lvl_desc.col_to_line_description(col) == line_to_description(&self.col_to_line(col)) {
            for i in 0..self.height {
                if self.get(col, i) == CellState::Empty {
                    self.set_no_update(col, i, CellState::Crossed)
                }
            }
        }
        if self.lvl_desc.row_to_line_description(row) == line_to_description(&self.row_to_line(row)) {
            for i in 0..self.height {
                if self.get(i, row) == CellState::Empty {
                    self.set_no_update(i, row, CellState::Crossed)
                }
            }
        }
    }

}

fn line_to_description(line: &Vec<CellState>) -> Vec<usize> {
    let mut result = Vec::new();
    let mut buffer = 0;
    for cell in line.iter() {
            if *cell != CellState::Filled {
                if buffer != 0 {
                    result.push(buffer);
                    buffer = 0
                }
            } else {
                buffer += 1;
            }
    }
    if buffer != 0 {
        result.push(buffer);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_to_description() {
        use CellState::*;
        let line = vec![Filled, Filled, Empty, Filled, Filled, Filled];
        let result = line_to_description(&line);
        assert_eq!(result, vec![2,3])
    }

    #[test]
    fn test_line_to_description_with_crosses() {
        use CellState::*;
        let line = vec![Empty, Filled, Crossed, Crossed, Filled, Filled, Filled, Empty, Crossed, Filled];
        let result = line_to_description(&line);
        assert_eq!(result, vec![1,3,1])
    }

}


