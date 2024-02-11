use ggez::input::gamepad::gilrs::GilrsBuilder;

use crate::description::LevelDescription;
use crate::grid::Grid;
use crate::transaction::{Transaction, TransactionBuilder};


#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Empty,
    Filled,
    Crossed
}

pub struct  GameGridState {
    cells: Vec<Vec<CellState>>, // в ГРИДЕ хранятся СТРОКИ блять
    height: usize,
    width: usize,
}
pub struct GameState {
    lvl_desc: LevelDescription,
    grid: GameGridState,
    move_queue: Vec<Transaction>
}

impl Grid for GameGridState {
    fn set(&mut self, col: usize, row: usize, state: CellState) { self.cells[row][col] = state; }
    fn get(&self, col: usize, row: usize) -> CellState { self.cells[row][col] }
    fn height(&self) -> usize { self.height }
    fn width(&self) -> usize { self.width }
}

impl GameState {
    pub fn new(lvl_desc: LevelDescription) -> Self {
        let width = lvl_desc.cols.len();
        let height = lvl_desc.rows.len();
        let cells = vec![vec![CellState::Empty; width]; height];
        let grid = GameGridState {cells, width, height};
        let move_queue = Vec::<Transaction>::new();
        Self { lvl_desc, grid, move_queue}
    }

    pub fn apply_transaction(&mut self, transaction: &Transaction) {

        if transaction.changes.len() > 0 {
            self.grid.apply_transaction(transaction);
            self.move_queue.push(transaction.clone());

            let mut builder = TransactionBuilder::new(&self.grid);

            for change in transaction.changes.iter() {
                if (change.new_state == CellState::Filled || change.new_state == CellState::Empty) {
                    update_nonogram(&mut builder, &self.lvl_desc, change.col, change.row);
                    update_level_description(&builder, &mut self.lvl_desc, change.col, change.row);
                }
            }

            let transaction = builder.to_transaction(&self.grid);

            if transaction.changes.len() > 0 {
                self.grid.apply_transaction(&transaction);
                self.move_queue.push(transaction);
            }
        }
    }
    pub fn set(&mut self, col: usize, row: usize, val: CellState) {
        let mut builder = TransactionBuilder::new(&self.grid);

        builder.set(col, row, val);

        let transaction = builder.to_transaction(&self.grid);

        self.grid.apply_transaction(&transaction);
        self.move_queue.push(transaction);

        if val == CellState::Filled || val == CellState::Empty {
            let mut builder = TransactionBuilder::new(&self.grid);

            update_nonogram(&mut builder, &self.lvl_desc, col, row);
            update_level_description(&builder, &mut self.lvl_desc, col, row);

            let transaction = builder.to_transaction(&self.grid);

            if transaction.changes.len() > 0 {
                self.grid.apply_transaction(&transaction);
                self.move_queue.push(transaction);
            }
        }

    }

    pub fn get(&self, col: usize, row: usize) -> CellState {
        self.grid.get(col, row)
    }

    pub fn height(&self) -> usize { self.grid.height() }
    pub fn width(&self) -> usize { self.grid.width() }

    pub fn undo(&mut self) {
        let transaction_option = self.move_queue.pop();

        if let Some(transaction) = transaction_option {
            self.grid.rollback_transaction(&transaction);
            for change in transaction.changes.iter() {
                update_level_description(&self.grid, &mut self.lvl_desc, change.col, change.row);
            }
        }
    }
    fn set_no_update(&mut self, col: usize, row: usize, val: CellState) {
        self.grid.set(col, row, val);
    }

    pub fn lvl_desc(&self) -> &LevelDescription {
        &(self.lvl_desc)
    }
    pub fn grid(&self) -> &GameGridState {
        &self.grid
    }
    pub fn grid_to_iter(&self) -> impl Iterator<Item = (usize, usize, CellState)> + '_ {
        self.grid.iter()
    }
}

pub fn update_level_description<T: Grid>(target: &T, lvl_desc: &mut LevelDescription, col: usize, row: usize) {
    if lvl_desc.col_to_line_description(col) == line_to_line_description(&target.col_to_line(col)) {
        for mut i in lvl_desc.cols[col].parts.iter_mut() {
            i.is_completed = true
        }
    }
    else {
        for mut i in lvl_desc.cols[col].parts.iter_mut() {
            i.is_completed = false
        }
    }

    if lvl_desc.row_to_line_description(row) == line_to_line_description(&target.row_to_line(row)) {
        for mut i in lvl_desc.rows[row].parts.iter_mut() {
            i.is_completed = true
        }
    }
    else {
        for mut i in lvl_desc.rows[row].parts.iter_mut() {
            i.is_completed = false
        }
    }
}
pub fn update_nonogram<T: Grid>(target: &mut T, lvl_desc: &LevelDescription, col: usize, row: usize) {
    if lvl_desc.col_to_line_description(col) == line_to_line_description(&target.col_to_line(col)) {
        for i in 0..target.height() {
            if target.get(col, i) == CellState::Empty {
                target.set(col, i, CellState::Crossed);
            }
        }
    }

    if lvl_desc.row_to_line_description(row) == line_to_line_description(&target.row_to_line(row)) {
        for i in 0..target.width() {
            if target.get(i, row) == CellState::Empty {
                target.set(i, row, CellState::Crossed);
            }
        }
    }
}
pub fn line_to_line_description(line: &Vec<CellState>) -> Vec<usize> {
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
        let result = line_to_line_description(&line);
        assert_eq!(result, vec![2,3])
    }

    #[test]
    fn test_line_to_description_with_crosses() {
        use CellState::*;
        let line = vec![Empty, Filled, Crossed, Crossed, Filled, Filled, Filled, Empty, Crossed, Filled];
        let result = line_to_line_description(&line);
        assert_eq!(result, vec![1,3,1])
    }

}


