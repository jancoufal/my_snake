use std::ops::{Add, Sub, Div, Mul};
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> where T: Copy {
    pub fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x, y }
    }

    pub fn new_from_array(xy: [T; 2]) -> Point2D<T> {
        Point2D { x: xy[0], y: xy[1] }
    }

    pub fn as_array(&self) -> [T; 2] {
        [self.x, self.y]
    }
}

impl<T: PartialEq> PartialEq for Point2D<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Add for Point2D<T> where T: Add<Output=T> {
    type Output = Point2D<T>;
    fn add(self, other: Point2D<T>) -> Point2D<T> {
        Point2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Point2D<T> where T: Sub<Output=T> {
    type Output = Point2D<T>;
    fn sub(self, other: Point2D<T>) -> Point2D<T> {
        Point2D {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul for Point2D<T> where T: Mul<Output=T> {
    type Output = Point2D<T>;
    fn mul(self, other: Point2D<T>) -> Point2D<T> {
        Point2D {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl<T> Div for Point2D<T> where T: Div<Output=T> {
    type Output = Point2D<T>;
    fn div(self, other: Point2D<T>) -> Point2D<T> {
        Point2D {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}