use std::{array, fmt::{Debug, Display}, cmp::{min, max}, borrow::BorrowMut};

use self::{row::Row, cell::Cell, cursor::Cursor};

pub mod cell;
pub mod row;
mod cursor;

pub type Board<const WIDTH: usize, const HEIGHT: usize> = [Row<WIDTH>; HEIGHT];

pub enum CursorPosChangeDir{
    Grow,
    Shrink
}
pub enum CursorRotChangeDir{
    Clockwise,
    CounterClockwise
}

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
        let mut start = HEIGHT;
        '_column_loop: for x in 0..WIDTH{
            start = (1..HEIGHT).
                find(
                    |y| {
                        if let (cell::Cell::Filled(_), cell::Cell::Empty) = (&self.board[*y - 1][x], &self.board[*y][x]) {
                            return true;
                        }
                        return false;
                    }
                )
                .unwrap_or(HEIGHT);
            
            'row_loop: for y in start..HEIGHT{
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

        if start != HEIGHT {
            (self.new_row)(&mut self.board[HEIGHT-1]);
        }
    }

    pub fn change_cursor_length(&mut self, dir: CursorPosChangeDir) {
        match dir{
            CursorPosChangeDir::Grow => {
                let mut new_cursor = self.cursor;
                new_cursor.double();

                if HEIGHT < new_cursor.length || WIDTH < new_cursor.length {
                    return ;
                }

                Self::fix_cursor_pos(&mut new_cursor);

                self.cursor = new_cursor;
            },
            CursorPosChangeDir::Shrink => {
                self.cursor.shrink();
            },
        }
    }

    pub fn move_cursor(&mut self, delta_x: i8, delta_y: i8) {

        let mut new_cursor = self.cursor;

        new_cursor.pivot.0 = self.cursor.pivot.0 + delta_x;
        new_cursor.pivot.1 = self.cursor.pivot.1 + delta_y;

        Self::fix_cursor_pos(&mut new_cursor);

        self.cursor = new_cursor;
    }

    fn fix_cursor_pos(cursor: &mut Cursor) {
        Self::fix_cursor_x_axis(cursor);
        
        Self::fix_cursor_y_axis(cursor);
    }

    fn fix_cursor_x_axis(cursor: &mut Cursor) {
        let mut end = cursor.end();

        let min = min(cursor.pivot.0, end.0);
        let max = max(cursor.pivot.0, end.0);

        if min < 0 {
            cursor.pivot.0 += 0 - min;
            end = cursor.end();
        }
        else if WIDTH <= max as usize {
            cursor.pivot.0 += (WIDTH - 1) as i8 - max;
            end = cursor.end();
        }
    }

    fn fix_cursor_y_axis(cursor: &mut Cursor) {
        let mut end = cursor.end();
        
        let min = min(cursor.pivot.1, end.1);
        let max = max(cursor.pivot.1, end.1);

        if min < 0 {
            cursor.pivot.1 += 0 - min;
        }
        else if HEIGHT <= max as usize {
            cursor.pivot.1 += (HEIGHT - 1) as i8 - max;
        }
    }

    pub fn change_cursor_rot(&mut self, rot: CursorRotChangeDir){
        let mut new_cursor = self.cursor;

        match rot {
            CursorRotChangeDir::Clockwise => new_cursor.clockwise_rot(),
            CursorRotChangeDir::CounterClockwise => new_cursor.counter_clockwise_rot(),
        }

        Self::fix_cursor_pos(&mut new_cursor);

        self.cursor = new_cursor;
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Game<WIDTH, HEIGHT> {
    fn default() -> Self {
        let board: [Row<WIDTH>; HEIGHT] = array::from_fn(|_| Row::default());
        
        Game{
            board: board,
            new_row: Row::random,
            cursor: Cursor::default(),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Debug for Game<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cursor_range = {

            let start = self.cursor.pivot;
            let end = self.cursor.end();

            (
                (min(start.0, end.0), min(start.1, end.1)),
                (max(start.0, end.0), max(start.1, end.1))
            )
        };

        let board = self.board
            .iter()
            .map(|row| format!("{:?}", row))
            .enumerate()
            .rev()
            .map(
                |(y, row)|{
                    let mut row = row; 

                    if cursor_range.0.1 as usize <= y &&  y <= cursor_range.1.1 as usize {

                        let mut tmp = String::from("");
                    
                        let mut chars = row.chars();
                        for i in 0..(cursor_range.0.0 as usize) {
                            tmp.push(chars.next().unwrap());
                        }

                        for x in (cursor_range.0.0 as usize)..=(cursor_range.1.0 as usize) {
                            tmp.push('X');
                            chars.next();
                        }

                        chars.for_each(|c| tmp.push(c));

                        row = tmp;
                    }
                    (y, row)
                }
            )
            .fold(
                format!("DEBUG{}GAME", " ".repeat(WIDTH-"DEBUG".len()+4)),
                |acc, row| format!("{}\n{}", acc, row.1)
            );
        write!(f, "{}\nCursor:{:?}", board, self.cursor)
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