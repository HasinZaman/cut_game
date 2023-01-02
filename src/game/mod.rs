use std::{array, fmt::{Debug, Display}};

use self::{row::Row, cell::Cell, cursor::Cursor};

pub mod cell;
pub mod row;
mod cursor;

pub type Board<const WIDTH: usize, const HEIGHT: usize> = [Row<WIDTH>; HEIGHT];

pub struct Game<const WIDTH: usize, const HEIGHT: usize> {
    pub board: Board<WIDTH, HEIGHT>,
    pub new_row: fn(&mut Row<WIDTH>),
    cursor: Cursor,
}

impl<const WIDTH: usize, const HEIGHT: usize> Game<WIDTH, HEIGHT> {
    pub fn new(new_row: fn(&mut Row<WIDTH>), cursor: Cursor) -> Self {
        let board: [Row<WIDTH>; HEIGHT] = array::from_fn(|_| Row::default());
        
        Game{
            board: board,
            new_row,
            cursor
        }
    }

    pub fn fall(&mut self) {
        'column_loop: for x in 0..WIDTH{
            if let (cell::Cell::Filled(_), cell::Cell::Filled(_)) = (&self.board[0][x], &self.board[1][x]) {
                continue 'column_loop;
            }
            'row_loop: for y in 2..HEIGHT{
                match (&self.board[y - 1][x], &self.board[y][x]) {
                    //next row
                    (cell::Cell::Filled(_), cell::Cell::Empty) |
                    (cell::Cell::Empty, cell::Cell::Empty) => {
                        continue 'row_loop;
                    }

                    //falling logic
                    (cell::Cell::Filled(_), cell::Cell::Filled(_)) |
                    (cell::Cell::Empty, cell::Cell::Filled(_)) => {
                        self.board[y - 1][x] = self.board[y][x];

                        self.board[y][x] = Cell::Empty;
                    },
                }
            }
        }

        (self.new_row)(&mut self.board[HEIGHT-1]);
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Game<WIDTH, HEIGHT> {
    fn default() -> Self {
        let board: [Row<WIDTH>; HEIGHT] = array::from_fn(|_| Row::default());
        
        Game{
            board: board,
            new_row: Row::empty,
            cursor: Cursor::default(),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Debug for Game<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.board
            .iter()
            .rev()
            .fold(
                String::from("Game"),
                |acc, row| format!("{}\n{:?}", acc, row)
            );
        write!(f, "{}", board)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Display for Game<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.board
            .iter()
            .rev()
            .fold(
                String::from("Game"),
                |acc, row| format!("{}\n{}", acc, row)
            );
        write!(f, "{}", board)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Iterator for Game<WIDTH, HEIGHT> {
    type Item = Board<WIDTH, HEIGHT>;

    fn next(&mut self) -> Option<Self::Item> {

        self.fall();

        Some(self.board)
    }
}