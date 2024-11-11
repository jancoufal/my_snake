use crate::point_2d::Point2D;

pub struct Matrix2D<T> {
    size: Point2D<i32>,
    data: Vec<T>,
}

impl<T: Default + Clone> Matrix2D<T> {
    pub fn new(size: &Point2D<i32>) -> Result<Matrix2D<T>, String> {
        if size.x <= 0 || size.y <= 0 {
            return Err(format!("Invalid dimensions {}x{}", size.x, size.y));
        }

        let total_size = (size.x * size.y) as usize;

        Ok(Matrix2D {
            size: *size,
            data: vec![T::default(); total_size],
        })
    }

    fn pos_to_index(&self, pos: Point2D<i32>) -> Option<usize> {
        if pos.x >= 0 && pos.x < self.size.x && pos.y >= 0 && pos.y < self.size.y {
            Some((pos.x * self.size.x + pos.y) as usize)
        } else {
            None
        }
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.data.get_mut(index)
    }
    
    pub fn get_pos(&self, pos: Point2D<i32>) -> Option<&T> {
        self.pos_to_index(pos).and_then(|i| self.get(i))
    }

    pub fn get_pos_mut(&mut self, pos: Point2D<i32>) -> Option<&mut T> {
        self.pos_to_index(pos).and_then(|i| self.get_mut(i))
    }
}
