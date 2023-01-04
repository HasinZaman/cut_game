use std::fmt::{Debug, Display};

use rand::{distributions::Standard, prelude::Distribution, Rng};

#[derive(Clone, Copy)]
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
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "E"),
            Self::Filled(val) => write!(f, "{}", *val as u8),
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
            }
        )
        
    }
}

impl Distribution<Cell> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cell {
        match rng.gen_range(0usize..=1usize) {
            0..=1 => Cell::Filled(rand::random()),
            _ => panic!()
        }
    }
}
