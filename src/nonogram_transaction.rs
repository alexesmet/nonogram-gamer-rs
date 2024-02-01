use std::cell::Cell;
use std::iter::{FlatMap, IntoIterator, Map};
use std::ops::Range;
use crate::grid::{CellState, GameGridState, GameState};
use crate::nonogram_target::NonogramTarget;

pub struct NonogramTransactionDetails {
    pub col: usize,
    pub row: usize,
    pub old_state: CellState,
    pub new_state: CellState
}
pub struct NonogramTransaction {
    pub changes: Vec<NonogramTransactionDetails>
}
impl NonogramTransaction {
    fn new() -> Self {
        Self { changes: Vec::<NonogramTransactionDetails>::new()}
    }
    fn set(&mut self, col: usize, row: usize, new_state: CellState, old_state: CellState) {
        self.changes.push(NonogramTransactionDetails {col, row, old_state, new_state})
    }
}
pub struct NonogramTransactionBuilder {
    grid: Vec<Vec<CellState>>,
    height: usize,
    width: usize
}
impl NonogramTarget for NonogramTransactionBuilder {
    fn set(&mut self, col: usize, row: usize, state: CellState) { self.grid[row][col] = state; }
    fn get(&self, col: usize, row: usize) -> CellState { self.grid[row][col] }
    fn height(&self) -> usize { self.height }
    fn width(&self) -> usize { self.width }
}

impl NonogramTransactionBuilder {
    pub fn new<T: NonogramTarget>(target: &T) -> Self {
        let mut grid = vec![vec![CellState::Empty; target.width()]; target.height()];
        for col in 0..target.width() {
            for row in 0..target.height() {
                grid[row][col] = target.get(col, row);
            }
        }
        Self { grid, height: target.height(), width: target.width() }
    }
    pub fn to_transaction<T: NonogramTarget>(&self, target: &T) -> NonogramTransaction {
        let mut transaction = NonogramTransaction::new();
        for col in 0..target.width() {
            for row in 0..target.height() {
                let old_state = target.get(col, row);
                let new_state = self.get(col, row);
                if old_state != new_state {
                    transaction.set(col, row, new_state, old_state);
                }
            }
        }
        transaction
    }
}
