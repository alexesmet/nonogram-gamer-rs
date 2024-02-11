use std::cell::Cell;
use std::iter::{FlatMap, IntoIterator, Map};
use std::ops::Range;
use crate::game_state::{CellState, GameGridState, GameState};
use crate::grid::Grid;

#[derive(Clone)]
pub struct TransactionDetails {
    pub col: usize,
    pub row: usize,
    pub old_state: CellState,
    pub new_state: CellState
}

#[derive(Clone)]
pub struct Transaction {
    pub changes: Vec<TransactionDetails>
}

impl Transaction {
    fn new() -> Self {
        Self { changes: Vec::<TransactionDetails>::new()}
    }
    fn set(&mut self, col: usize, row: usize, new_state: CellState, old_state: CellState) {
        self.changes.push(TransactionDetails {col, row, old_state, new_state})
    }
}
pub struct TransactionBuilder {
    grid: Vec<Vec<CellState>>,
    height: usize,
    width: usize
}
impl Grid for TransactionBuilder {
    fn set(&mut self, col: usize, row: usize, state: CellState) { self.grid[row][col] = state; }
    fn get(&self, col: usize, row: usize) -> CellState { self.grid[row][col] }
    fn height(&self) -> usize { self.height }
    fn width(&self) -> usize { self.width }
}

impl TransactionBuilder {
    pub fn new<T: Grid>(target: &T) -> Self {
        let mut grid = vec![vec![CellState::Empty; target.width()]; target.height()];
        for col in 0..target.width() {
            for row in 0..target.height() {
                grid[row][col] = target.get(col, row);
            }
        }
        Self { grid, height: target.height(), width: target.width() }
    }
    pub fn to_transaction<T: Grid>(&self, target: &T) -> Transaction {
        let mut transaction = Transaction::new();
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
