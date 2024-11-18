use crate::matrix_2d::Matrix2D;
use crate::point_2d::Point2D;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy)]
pub struct Cell {
    pub pos: Point2D<i32>,  // self xy position
}

impl Default for Cell {
    fn default() -> Self {
        println!("Cell::default()");
        Cell{
            pos: Point2D::new(-1, -1),
        }
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        println!("Cell::clone()");
        Cell { pos: self.pos }
    }
}

pub struct GameStateX {
    pub board: Matrix2D<Cell>,
}

impl GameStateX {
    pub fn new(board_size: Point2D<i32>) -> Result<GameStateX, String> {
        // if board_size.x < 5 || board_size.y < 5 {
        //     return Err(format!("Game field must be 5x5 cells at least. {}x{} entered.", board_size.x, board_size.y));
        // }

        // put snake head in the middle
        let snake_head_index = (board_size.x * board_size.y) / 2 + board_size.x / 2;

        let board = Matrix2D::new(&board_size)?;

        Ok(GameStateX {
            board
        })
    }
}