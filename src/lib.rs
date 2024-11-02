mod point_2d;

use std::cell::RefCell;
use std::cmp::PartialEq;
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
                square_size.x as f64,
                square_size.y as f64,
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
    link_to_tail: Option<Rc<Cell>>,
}

#[derive(Debug)]
pub struct Game {
    pub state: GameState,
    pub field_size: Point2D<usize>,
    pub field: Vec<Vec<Rc<RefCell<Cell>>>>,
    pub snake_head: Rc<RefCell<Cell>>,
    pub direction: Direction,
    pub snake_length: usize,
    pub step_count: usize,
}

impl Game {
    pub fn new(cols: usize, rows: usize) -> Result<Game, String> {

        if cols < 5 || rows < 5 {
            return Err(format!("Game field must be 5x5 cells at least. {}x{} entered.", cols, rows));
        }

        let snake_head_pos = Point2D::new((cols / 2) as i32, (rows / 2) as i32);
        let mut snake_head: Option<Rc<RefCell<Cell>>> = None;

        let mut field: Vec<Vec<Rc<RefCell<Cell>>>> = Vec::new();
        for r in 0..rows {
            let mut cell_row: Vec<Rc<RefCell<Cell>>> = Vec::new();
            for c in 0..cols {
                let pos = Point2D::new(r as i32, c as i32);
                let new_cell = Rc::new(RefCell::new(Cell {
                    pos,
                    cell_type: match (c, r) {
                        (0, _) => CellType::Border,
                        (_, _) if c == cols-1 => CellType::Border,
                        (_, 0) => CellType::Border,
                        (_, _) if r == rows-1 => CellType::Border,
                        (_, _) if pos == snake_head_pos => CellType::Snake(SnakeBodyPart::Head(1)),
                        (_, _) => CellType::Empty,
                    },
                    link_to_tail: None,
                }));
                
                if new_cell.borrow().cell_type == CellType::Snake(SnakeBodyPart::Head(r)) {
                    snake_head = Some(Rc::clone(&new_cell));
                }

                cell_row.push(new_cell);
            }
            field.push(cell_row);
        }

        Ok(Game {
            state: GameState::Paused,
            field_size: Point2D::new(cols, rows),
            field,
            snake_head: snake_head.unwrap(),
            direction: Direction::Right,
            snake_length: 1usize,
            step_count: 0usize,
        })
    }

    pub fn cell_iter(&self) -> CellIterator {
        CellIterator {
            game: self,
            iter_position: Point2D::new(0, 0),
        }
    }

    pub fn play(&mut self, direction: Direction) {
        self.state = GameState::Playing;
        self.direction = direction;
    }

    pub fn set_movement_direction(&mut self, direction: Direction) {
        // we cannot move in opposite direction
        self.direction = match (self.direction, direction) {
            (Direction::Right, Direction::Left) => Direction::Right,
            (Direction::Left, Direction::Right) => Direction::Left,
            (Direction::Up, Direction::Down) => Direction::Up,
            (Direction::Down, Direction::Up) => Direction::Down,
            (_, _) => direction,
        };
    }

    pub fn update_game_state(&mut self) {
        let new_head_pos = self.snake_head.borrow().pos + match self.direction {
            Direction::Right => Point2D::new(1i32, 0i32),
            Direction::Left => Point2D::new(-1i32, 0i32),
            Direction::Up => Point2D::new(0i32, -1i32),
            Direction::Down => Point2D::new(0i32, 1i32),
        };

        // todo: collision (border hit, self bite, apple)

        // move snake body
        let mut new_head_cell = self.field
            .get_mut(new_head_pos.y as usize).unwrap()
            .get_mut(new_head_pos.x as usize).unwrap();
        
        new_head_cell.borrow_mut().cell_type = CellType::Empty;
        // new_head_cell.cell_type = self.snake_head.cell_type;
    }
}

pub struct CellIterator<'a> {
    game: &'a Game,
    iter_position: Point2D<usize>,
}

pub struct CellContext {
    pub cell: Rc<RefCell<Cell>>,
    pub cell_position: Point2D<Scalar>,
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

