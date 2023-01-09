use std::fmt::Debug;

use super::Game;

#[derive(Clone, Copy)]
pub struct Score {
    pub cols: u64,
    pub col_modifier: u64,
    pub rows: u64,
    pub row_modifier: u64,
}

impl Score{
    pub fn new(col_modifier: u64, row_modifier: u64) -> Self {
        Self {
            cols: 0,
            col_modifier,
            rows: 0,
            row_modifier
        }
    }
    
    pub fn score(&self) -> u64 {
        self.cols * self.col_modifier + self.rows * self.row_modifier
    }

}

impl<const W: usize, const H: usize> From<Game<W, H>> for Score {
    fn from(value: Game<W, H>) -> Self {
        let (cols, rows) = value.completed_lines();

        Self { cols: cols, col_modifier: 1, rows: rows, row_modifier: 1 }
    }
}

impl Debug for Score{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Score")
            .field("score", &self.score())
            .field("cols", &self.cols)
            .field("col_modifier", &self.col_modifier)
            .field("rows", &self.rows)
            .field("row_modifier", &self.row_modifier)
            .finish()
    }
}

impl Default for Score{
    fn default() -> Self {
        Self::new(1,1)
    }
}