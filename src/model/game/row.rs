use std::{
    ops::{Deref, DerefMut},
    fmt::{Debug, Display}, array, mem
};

use super::cell::Cell;

#[derive(Clone, Copy)]
pub struct Row<const WIDTH: usize>([Cell; WIDTH]);

impl<const WIDTH: usize> Row<WIDTH> {
    pub fn random(row: &mut Row<WIDTH>) {
        for i in 0..WIDTH{
            row[i] = rand::random();
        }
    }
    
    pub fn empty(row: &mut Row<WIDTH>) {
        for i in 0..WIDTH {
            row[i] = Cell::Empty;
        }
    }
    
    pub fn swap(row_1: &mut Row<WIDTH>, row_2: &mut Row<WIDTH>) {
        mem::swap(row_1, row_2);
    }
}

impl<const WIDTH: usize> Debug for Row<WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let debug = (*self).iter()
            .fold(String::from(""), |acc, cell| format!("{}{:?}", acc, cell));

        let game_view = self.to_string();
        write!(f, "{debug}\t{game_view}")
    }
}

impl<const WIDTH: usize> Display for Row<WIDTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row = (*self).iter()
            .fold(String::from(""), |acc, cell| format!("{}{}", acc, cell));
        write!(f, "{}", row)
    }
}

impl<const WIDTH: usize> Default for Row<WIDTH> {
    fn default() -> Self {
        let row: [Cell; WIDTH] = array::from_fn(|_| Cell::Empty);

        Row(row)
    }
}

impl<const WIDTH: usize> Deref for Row<WIDTH>{
    type Target = [Cell; WIDTH];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const WIDTH: usize> DerefMut for Row<WIDTH>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

