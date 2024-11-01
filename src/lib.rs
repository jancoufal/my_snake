mod point_2d;
use crate::point_2d::Point2D;

use graphics::math::Scalar;
use graphics::rectangle;
use graphics::types::Rectangle;
use piston::Size;
use std::rc::Rc;

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
pub enum CellType {
    Empty,
    Border,
    SnakeHead,
    SnakeBody,
    SnakeTail,
    Food,
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
                square_size.x as f64,
                square_size.y as f64,
            )
        }
    }

    pub fn get_viewport_size(&self) -> Size {
        Size { width: self.viewport_size.x as f64, height: self.viewport_size.y as f64 }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub int_code: i32,
}

impl Cell {
    pub fn new_border() -> Cell { Cell { int_code: CELL_CODE_BORDER } }
    pub fn new_empty() -> Cell { Cell { int_code: CELL_CODE_EMPTY } }
    pub fn new_food() -> Cell { Cell { int_code: CELL_CODE_FOOD } }
    pub fn new_head() -> Cell { Cell { int_code: CELL_CODE_INIT_HEAD } }

    pub fn get_type(&self, snake_length: i32) -> CellType {
        match self.int_code {
            // CELL_CODE_EMPTY => CellType::Empty,
            // CELL_CODE_BORDER => CellType::Border,
            // CELL_CODE_FOOD => CellType::Food,
            0 => CellType::Empty,
            -1 => CellType::Border,
            -2 => CellType::Food,
            _ if self.int_code == snake_length => CellType::SnakeHead,
            1 => CellType::SnakeTail, // todo: not really true
            _ => CellType::SnakeBody,
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub state: GameState,
    pub field_size: Point2D<usize>,
    pub field: Vec<Vec<Rc<Cell>>>,
    pub snake: Vec<Rc<Cell>>,
    pub direction: Direction,
    pub snake_length: usize,
    pub step_count: usize,
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Game {

        let snake_head = Rc::new(Cell::new_head());

        let mut snake: Vec<Rc<Cell>> = Vec::new();
        snake.push(Rc::clone(&snake_head));

        let mut field: Vec<Vec<Rc<Cell>>> = Vec::new();
        for r in 0..rows {
            let mut cell_row: Vec<Rc<Cell>> = Vec::new();
            for c in 0..cols {
                let new_cell = match (c, r) {
                    (0, _) => Rc::new(Cell::new_border()),
                    (_, _) if c == cols-1 => Rc::new(Cell::new_border()),
                    (_, 0) => Rc::new(Cell::new_border()),
                    (_, _) if r == rows-1 => Rc::new(Cell::new_border()),
                    (_, _) if c == cols/2 && r == rows/2 => Rc::clone(&snake_head),
                    (_, _) => Rc::new(Cell::new_empty()),
                };
                cell_row.push(new_cell);
            }
            field.push(cell_row);
        }

        Game {
            state: GameState::Paused,
            field_size: Point2D::new(cols, rows),
            field,
            snake,
            direction: Direction::Right,
            snake_length: 1usize,
            step_count: 0usize,
        }
    }

    pub fn play(&mut self, direction: Direction) {
        self.state = GameState::Playing;
        self.direction = direction;
    }

    pub fn cell_iter(&self) -> CellIterator {
        CellIterator {
            game: self,
            iter_position: Point2D::new(0, 0),
        }
    }
}

pub struct CellIterator<'a> {
    game: &'a Game,
    iter_position: Point2D<usize>,
}

impl<'a> Iterator for CellIterator<'a> {
    type Item = CellContext;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter_position.y >= self.game.field_size.y {
            return None;
        }

        let cell = self.game.field[self.iter_position.y][self.iter_position.x].clone();
        let cell_context = CellContext {
            cell,
            cell_position: Point2D::new(self.iter_position.x as Scalar, self.iter_position.y as Scalar),
        };

        // move to the next cell
        self.iter_position.x += 1;
        if self.iter_position.x >= self.game.field[self.iter_position.y].len() {
            self.iter_position.x = 0;
            self.iter_position.y += 1;
        }

        Some(cell_context)
    }
}

pub struct CellContext {
    pub cell: Rc<Cell>,
    pub cell_position: Point2D<Scalar>,
}
