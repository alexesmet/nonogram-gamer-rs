use crate::grid::CellState;
use crate::nonogram_transaction::{NonogramTransaction};

pub struct NonogramTargetIterator<'a, T: NonogramTarget> {
    target: &'a T,
    first: bool,
    row: usize,
    col: usize
}

impl<'a, T: NonogramTarget> Iterator for NonogramTargetIterator<'a, T> {
    type Item = (usize, usize, CellState);

    fn next(&mut self) -> Option<Self::Item> {
        if self.first {
            self.first = false
        }
        else if self.col < self.target.width() - 1 {
            self.col += 1;
        }
        else if self.row < self.target.height() - 1 {
            self.col = 0;
            self.row += 1;
        }
        else {
            return None;
        }

        Some((self.col, self.row, self.target.get(self.col, self.row)))
    }
}
pub trait NonogramTarget {
    fn set(&mut self, col: usize, row: usize, state: CellState);
    fn get(&self, col: usize, row: usize) -> CellState;
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn row_to_line(&self, row: usize) -> Vec<CellState> {
        (0..self.width()).into_iter().map(|i| self.get(i, row)).collect()
    }
    fn col_to_line(&self, col: usize) -> Vec<CellState> {
        (0..self.height()).into_iter().map(|i| self.get(col, i)).collect()
    }
    //MY GOD. Where is my `auto` keyword...
    fn iter(&self) -> NonogramTargetIterator<Self> where Self: Sized {
        NonogramTargetIterator {
            target: self,
            first: true,
            row: 0,
            col: 0
        }
        //let result: Vec<(usize, usize, CellState)>= (0..self.width()).flat_map(|col_id| (0..self.height()).map( |row_id| (col_id, row_id, self.get(col_id, row_id))) ).collect();
        //result.iter()
    }

    fn apply_transaction(&mut self, nonogram_transaction: &NonogramTransaction) {
        for change in nonogram_transaction.changes.iter() {
            self.set(change.col, change.row, change.new_state);
        }
    }

    fn rollback_transaction(&mut self, nonogram_transaction: &NonogramTransaction) {
        for change in nonogram_transaction.changes.iter() {
            self.set(change.col, change.row, change.old_state);
        }
    }
}