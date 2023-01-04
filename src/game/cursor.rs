use std::cmp::{max, min};

pub type Coord = (i8, i8);
type Rot = u8; // 0 = 0deg, 1 = 90deg, 2 = 180deg, 3 = 270deg

#[derive(Clone, Copy)]
pub struct Cursor {
    pub pivot: Coord,
    pub length: i8,
    rotation: Rot,
}

impl Cursor {
    pub fn clockwise_rot(&mut self) {
        self.rotation = (self.rotation + 1) % 4
    }
    pub fn counter_clockwise_rot(&mut self) {
        self.rotation = match self.rotation.checked_sub(1) {
            Some(val) => val,
            None => 3,
        }
    }
    pub fn end(&self) -> Coord{
        let mut end = self.pivot;
        end.0 += self.length as i8;

        let cos = cos(self.rotation);
        let sin = sin(self.rotation);

        let end: Coord = (
            end.0 * cos - end.1 * sin + self.pivot.0,
            end.0 * sin + end.1 * cos + self.pivot.1
        );

        end
    }
    pub fn double(&mut self) {
        if let Some(length) =  self.length.checked_mul(2) {
            self.length = length;
        }
    }
    pub fn shrink(&mut self) {
        if let Some(length) = self.length.checked_div(2) {
            if length != 0 {
                self.length = length;
            }
        }
    }
}

impl Default for Cursor{
    fn default() -> Self {
        Self {
            pivot: (0, 0),
            length: 1,
            rotation: 0,
        }
    }
}

fn cos(rot: Rot) -> i8 {
    match rot {
        1 | 3 => 0,
        0 => 1,
        2 => -1,
        _=> panic!()
    }
}

fn sin(rot: Rot) -> i8 {
    match rot {
        1  => 1,
        3 => -1,
        0 | 2 => 0,
        _=> panic!()
    }
}