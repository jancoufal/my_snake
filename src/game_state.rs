use std::cell::RefCell;
use std::rc::Rc;
use graphics::math::Scalar;
use crate::Game;
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
        Cell {
            pos: Point2D::new(-1, -1),
        }
    }
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        println!("Cell::clone({:?})", self);
        Cell { pos: self.pos.clone() }
    }
}


pub struct CellContext<'a> {
    pub cell_index: usize,
    pub cell: &'a Cell,
}

pub struct CellIterator<'a> {
    game: &'a GameStateX,
    index: usize,
}

impl<'a> CellIterator<'a> {
    pub fn new(game: &'a GameStateX) -> Self {
        CellIterator { game, index: 0 }
    }
}

impl<'a> Iterator for CellIterator<'a> {
    type Item = CellContext<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.game.board.data.len() {
            return None;
        }

        let r = Some(CellContext{
            cell_index: self.index,
            cell: &self.game.board.data[self.index]
        });

        self.index += 1;

        r
    }
}


// todo:x
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
    
    pub fn iter_cell_mut(&mut self) -> CellIterator {
        CellIterator::new(self)
    }
}