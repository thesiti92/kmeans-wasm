use serde::{Deserialize, Serialize};
use std::ops;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const ORIGIN: Point = Point { x: 0.0, y: 0.0 };
}

impl ops::Add for Point {
    type Output = Point;
    fn add(self, _rhs: Point) -> Point {
        Point {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl<'a, 'b> ops::Add<&'b Point> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Div<f32> for Point {
    type Output = Point;
    fn div(self, _rhs: f32) -> Point {
        Point {
            x: self.x / _rhs,
            y: self.y / _rhs,
        }
    }
}
