use crate::game_state::CellState;
use crate::grid::{Grid, GridIterator};

pub struct LineIterator<'a, T: Line> {
    target: &'a T,
    pos: usize
}

impl<'a, T: Line> Iterator for LineIterator<'a, T> {
    type Item = CellState;
    fn next(&mut self) -> Option<Self::Item> {
        if (self.pos < self.target.len()) {
            let res = Some(self.target.get(self.pos));
            self.pos += 1;
            res
        }
        else {
            None
        }
    }
}
pub trait Line {
    fn get(&self, pos: usize) -> CellState;
    fn set(&mut self, pos: usize, cell_state: CellState);
    fn len(&self) -> usize;
    fn iter(&self) -> LineIterator<Self> where Self: Sized {
        LineIterator {
            pos: 0,
            target: &self
        }
    }
}

pub struct RowLine<'a, T : Grid> {
    target: &'a mut T,
    row_num: usize
}

impl<'a, T: Grid> RowLine<'a, T> {
    pub fn new(tagret: &'a mut T, row_num: usize) -> Self {
        Self {
            target: tagret,
            row_num
        }
    }
}

impl<'a, T: Grid> Line for RowLine<'a, T> {
    fn get(&self, pos: usize) -> CellState { self.target.get(pos, self.row_num) }

    fn set(&mut self, pos: usize, cell_state: CellState) { self.target.set(pos, self.row_num, cell_state) }

    fn len(&self) -> usize { self.target.width() }
}

pub struct ColLine<'a, T : Grid> {
    target: &'a mut T,
    col_num: usize
}

impl<'a, T: Grid> ColLine<'a, T> {
    pub fn new(target: &'a mut T, col_num: usize) -> Self {
        Self {
            target: target,
            col_num
        }
    }
}

impl<'a, T: Grid> Line for ColLine<'a, T> {
    fn get(&self, pos: usize) -> CellState { self.target.get(self.col_num, pos) }

    fn set(&mut self, pos: usize, cell_state: CellState) { self.target.set(self.col_num, pos, cell_state) }

    fn len(&self) -> usize { self.target.height() }
}