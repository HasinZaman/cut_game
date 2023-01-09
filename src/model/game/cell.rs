use std::fmt::{Debug, Display};

use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum CellValue {
    Zero = 0,
    One = 1,
}

impl From<usize> for CellValue {
    fn from(value: usize) -> Self {
        match value {
            0 => CellValue::Zero,
            1 => CellValue::One,
            _=> panic!("Invalid conversion")
        }
    }
}

impl Distribution<CellValue> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CellValue {
        rng.gen_range(0usize..=1usize).into()
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    Filled(CellValue),
    Random
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "E"),
            Self::Filled(val) => write!(f, "{}", *val as u8),
            Cell::Random => write!(f, "R"),
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cell::Empty => String::from(" "),
                Cell::Filled(val) => format!("{}", *val as u8),
                Cell::Random => String::from("R"),
            }
        )
        
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Empty, Self::Empty) => false,
            (Self::Filled(l0), Self::Filled(r0)) => l0 == r0,
            (Self::Random, Self::Filled(_)) | (Self::Filled(_), Self::Random) => true,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0usize..=10usize) {
            0..=2 => Cell::Empty,
            3..=7 => Cell::Filled(rand::random()),
            8..=10 => Cell::Random,
            _ => panic!()
        }
    }
}
