use std::fmt::Debug;

pub struct Score {
    cols: u64,
    col_modifier: u64,
    rows: u64,
    row_modifier: u64,
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

    pub fn add_row(&mut self) {
        self.rows+=1;
    }

    pub fn add_column(&mut self) {
        self.cols+=1;
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