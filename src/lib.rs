mod point_2d;

use std::cell::RefCell;
use std::cmp::PartialEq;
use crate::point_2d::Point2D;

use graphics::math::Scalar;
use graphics::rectangle;
use graphics::types::Rectangle;
use piston::Size;
use std::rc::Rc;
use opengl_graphics::{Texture, TextureSettings};

static CELL_CODE_INIT_HEAD: i32 = 1;
static CELL_CODE_EMPTY: i32 = 0;
static CELL_CODE_BORDER: i32 = -1;
static CELL_CODE_FOOD: i32 = -2;


#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
pub enum SnakeBodyPart {
    Head(usize),
    Body(usize),
    Tail(usize),
}

impl PartialEq for SnakeBodyPart {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SnakeBodyPart::Head(_), SnakeBodyPart::Head(_)) => true,
            (SnakeBodyPart::Body(_), SnakeBodyPart::Body(_)) => true,
            (SnakeBodyPart::Tail(_), SnakeBodyPart::Tail(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CellType {
    Uninitialized,  // for render optimization
    Empty,
    Border,
    Snake(SnakeBodyPart),
    Food,
}

impl PartialEq for CellType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CellType::Empty, CellType::Empty) => true,
            (CellType::Border, CellType::Border) => true,
            (CellType::Food, CellType::Food) => true,
            (CellType::Snake(part1), CellType::Snake(part2)) => part1 == part2,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum GameOverType {
    PlaygroundFilled,
    BorderHit,
    SelfBite
}

#[derive(Debug, Copy, Clone)]
pub enum GameState {
    Paused,
    Playing,
    GameOver(GameOverType),
}

pub struct RenderSettings {
    pub viewport_size: Point2D<usize>,
    pub grid_size: Point2D<usize>,
    pub square_size: Point2D<Scalar>,
    pub square: Rectangle,
}

impl RenderSettings {
    pub fn new(viewport_size: [usize; 2], grid_size: [usize; 2]) -> RenderSettings {
        let viewport_size = Point2D::new_from_array(viewport_size);
        let grid_size = Point2D::new_from_array(grid_size);
        let square_size = Point2D {
            x: viewport_size.x as Scalar / grid_size.x as Scalar,
            y: viewport_size.y as Scalar / grid_size.y as Scalar,
        };

        RenderSettings {
            viewport_size,
            grid_size,
            square_size,
            square: rectangle::rectangle_by_corners(
                0.0,
                0.0,
                square_size.x - 1.,
                square_size.y - 1.,
            )
        }
    }

    pub fn get_viewport_size(&self) -> Size {
        Size { width: self.viewport_size.x as f64, height: self.viewport_size.y as f64 }
    }
}

#[derive(Debug)]
pub struct Cell {
    pub pos: Point2D<i32>,
    pub cell_type: CellType,
    pub rendered_cell_type: CellType,   // for render optimization
    link_to_tail: Option<Rc<Cell>>,
}

#[derive(Debug)]
pub struct Game {
    pub state: GameState,
    pub field_size: Point2D<usize>,
    pub field: Vec<Rc<RefCell<Cell>>>,  // 2d array in single 1d container
    pub snake_head_index: usize,
    pub direction: Direction,
    pub snake_length: usize,
    pub step_count: usize,
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Result<Game, String> {

        if cols < 5 || rows < 5 {
            return Err(format!("Game field must be 5x5 cells at least. {}x{} entered.", cols, rows));
        }

        let mut field: Vec<Rc<RefCell<Cell>>> = Vec::with_capacity(cols * rows);
        
        // put snake head in the middle
        let snake_head_index = (cols * rows) / 2 + rows / 2;

        for index in 0..(cols * rows) {
            let pos = Point2D::new(index / cols, index % cols);
            let cell_type = match (pos.x, pos.y) {
                (0, _) => CellType::Border,
                (_, y) if y == cols-1 => CellType::Border,
                (_, 0) => CellType::Border,
                (x, _) if x == rows-1 => CellType::Border,
                (_, _) if index == snake_head_index => CellType::Snake(SnakeBodyPart::Head(1)),
                (_, _) => CellType::Empty,
            };

            let pos_i32: Point2D<i32> = Point2D::new(pos.x as i32, pos.y as i32);

            field.push(Rc::new(RefCell::new(Cell {
                pos: pos_i32,
                cell_type,
                rendered_cell_type: CellType::Uninitialized,
                link_to_tail: None,
            })));
        }
        
        Ok(Game {
            state: GameState::Paused,
            field_size: Point2D::new(cols, rows),
            field,
            snake_head_index,
            direction: Direction::Right,
            snake_length: 1usize,
            step_count: 0usize,
        })
    }

    pub fn get_field_index(&self, x: i32, y: i32) -> usize {
        y as usize * self.field_size.x + x as usize
    }

    pub fn get_index_from_point(&self, pos: Point2D<i32>) -> usize {
        pos.y as usize * self.field_size.x + pos.x as usize
    }

    pub fn get_point_from_index(&self, index: usize) -> Point2D<i32> {
        Point2D::new(
            index as i32 % self.field_size.x as i32,
            index as i32 / self.field_size.x as i32
        )
    }

    pub fn cell_iter_mut(&mut self) -> CellIterator {
        CellIterator {
            game: self,
            iter_index: 0,
        }
    }

    pub fn play(&mut self, direction: Direction) {
        self.state = GameState::Playing;
        self.direction = direction;
    }

    pub fn set_movement_direction(&mut self, direction: Direction) {
        // we cannot move in opposite direction when length > 1
        if self.field.get(self.snake_head_index).unwrap().borrow().cell_type.eq(&CellType::Snake(SnakeBodyPart::Head(1))) {
            self.direction = direction;
        }
        else {
            self.direction = match (self.direction, direction) {
                (Direction::Right, Direction::Left) => Direction::Right,
                (Direction::Left, Direction::Right) => Direction::Left,
                (Direction::Up, Direction::Down) => Direction::Up,
                (Direction::Down, Direction::Up) => Direction::Down,
                (_, _) => direction,
            };
        }
    }

    pub fn update_game_state(&mut self) {
        let head_pos_2d = self.get_point_from_index(self.snake_head_index);

        let mut new_head_pos_2d = head_pos_2d + match self.direction {
            Direction::Right => Point2D::new(1i32, 0i32),
            Direction::Left => Point2D::new(-1i32, 0i32),
            Direction::Up => Point2D::new(0i32, -1i32),
            Direction::Down => Point2D::new(0i32, 1i32),
        };

        // println!(
        //     "Game Update: Direction: {:?}, head_pos: {:?} -> {:?}",
        //     self.direction,
        //     head_pos_2d,
        //     new_head_pos_2d,
        // );

        // todo: collision (border hit, self bite, apple)

        // debug
        if self.get_index_from_point(new_head_pos_2d) >= self.field.len() {
            new_head_pos_2d = Point2D::new(0, 0);
        }

        // move snake body
        let new_head_cell = self.field
            .get(self.get_index_from_point(new_head_pos_2d))
            .unwrap();

        new_head_cell.borrow_mut().cell_type = CellType::Snake(SnakeBodyPart::Head(1));
        self.field.get(self.snake_head_index).unwrap().borrow_mut().cell_type = CellType::Empty;

        self.snake_head_index = self.get_index_from_point(new_head_pos_2d);
    }
}

pub struct CellIterator<'a> {
    game: &'a Game,
    iter_index: usize,
}

pub struct CellContext {
    pub cell: Rc<RefCell<Cell>>,
    pub cell_position: Point2D<Scalar>,
}

impl<'a> Iterator for CellIterator<'a> {
    type Item = CellContext;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_index >= self.game.field.len() {
            return None;
        }

        let cell = self.game.field[self.iter_index].clone();
        let cell_pos = self.game.get_point_from_index(self.iter_index);
        let cell_context = CellContext {
            cell,
            cell_position: Point2D::new(cell_pos.x as Scalar, cell_pos.y as Scalar),
        };

        // move to the next cell
        self.iter_index += 1;

        Some(cell_context)
    }
}

