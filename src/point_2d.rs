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
