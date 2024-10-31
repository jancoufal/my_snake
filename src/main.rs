extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::math::Scalar;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use std::cell::RefCell;
use piston::Size;

static CELL_CODE_INIT_HEAD: i32 = 1;
static CELL_CODE_EMPTY: i32 = 0;
static CELL_CODE_BORDER: i32 = -1;
static CELL_CODE_FOOD: i32 = -2;


#[derive(Debug, Copy, Clone)]
struct Point2D<T> {
    x: T,
    y: T,
}

impl<T> Point2D<T> where T: Copy {
    fn new(x: T, y: T) -> Point2D<T> {
        Point2D { x, y }
    }

    fn new_from_array(xy: [T; 2]) -> Point2D<T> {
        Point2D { x: xy[0], y: xy[1] }
    }

    fn as_array(&self) -> [T; 2] {
        [self.x, self.y]
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    None,
}

#[derive(Debug, Copy, Clone)]
enum CellType {
    Empty,
    Border,
    SnakeHead,
    SnakeBody,
    SnakeTail,
    Food,
}

#[derive(Debug, Copy, Clone)]
struct Cell {
    int_code: i32,
}

impl Cell {
    fn new_border() -> Cell { Cell { int_code: CELL_CODE_BORDER } }
    fn new_empty() -> Cell { Cell { int_code: CELL_CODE_EMPTY } }
    fn new_food() -> Cell { Cell { int_code: CELL_CODE_FOOD } }
    fn new_head() -> Cell { Cell { int_code: CELL_CODE_INIT_HEAD } }

    fn get_type(&self, snake_length: i32) -> CellType {
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

#[derive(Debug, Copy, Clone)]
enum GameOverType {
    PlaygroundFilled,
    BorderHit,
    SelfBite
}

#[derive(Debug, Copy, Clone)]
enum GameState {
    Paused,
    Playing,
    GameOver(GameOverType),
}

#[derive(Debug)]
struct Game {
    state: GameState,
    field_size: Point2D<usize>,
    field: Vec<Vec<RefCell<Cell>>>,
    // snake: Vec<Weak<RefCell<Cell>>>,
    direction: Direction,
    snake_length: usize,
    step_count: usize,
}

impl Game {
    fn new(cols: usize, rows: usize) -> Game {

        let mut field: Vec<Vec<RefCell<Cell>>> = Vec::new();
        for r in 0..rows {
            let mut cell_row: Vec<RefCell<Cell>> = Vec::new();
            for c in 0..cols {
                let new_cell = match (c, r) {
                    (0, _) => Cell::new_border(),
                    (_, _) if c == cols-1 => Cell::new_border(),
                    (_, 0) => Cell::new_border(),
                    (_, _) if r == rows-1 => Cell::new_border(),
                    (_, _) if c == cols/2 && r == rows/2 => Cell::new_head(),
                    (_, _) => Cell::new_empty(),
                };
                cell_row.push(RefCell::new(new_cell));
            }
            field.push(cell_row);
        }

        Game {
            state: GameState::Paused,
            field_size: Point2D::new(cols, rows),
            field,
            direction: Direction::None,
            snake_length: 1usize,
            step_count: 0usize,
        }
    }

    fn play(&mut self, direction: Direction) {
        self.state = GameState::Playing;
        self.direction = direction;
    }
}

struct RenderSettings {
    viewport_size: Point2D<usize>,
    grid_size: Point2D<usize>,
    square_size: Point2D<Scalar>,
}

impl RenderSettings {

    fn new(viewport_size: [usize; 2], grid_size: [usize; 2]) -> RenderSettings {
        let viewport_size = Point2D::new_from_array(viewport_size);
        let grid_size = Point2D::new_from_array(grid_size);
        RenderSettings {
            viewport_size,
            grid_size,
            square_size: Point2D {
                x: viewport_size.x as Scalar / grid_size.x as Scalar,
                y: viewport_size.y as Scalar / grid_size.y as Scalar,
            },
        }
    }

    fn get_viewport_size(&self) -> Size {
        Size { width: self.viewport_size.x as f64, height: self.viewport_size.y as f64 }
    }

    fn get_square_trans(&self, x_index: usize, y_index: usize) -> Point2D<Scalar> {
        Point2D {
            x: self.square_size.x * x_index as Scalar,
            y: self.square_size.y * y_index as Scalar,
        }
    }
}

struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,  // Rotation for the square.
}

impl App {
    fn render(&mut self, args: &RenderArgs, render_settings: &RenderSettings, game: &Game) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        const BROWN: [f32; 4] = [1.0, 0.5, 0.2, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const DARK: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
        const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let mut counter = 0;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GRAY, gl);

            for (y, field_row) in game.field.iter().enumerate() {
                for (x, cell) in field_row.iter().enumerate() {

                    let st = render_settings.get_square_trans(x, y);

                    let transform = c.transform
                        .trans(st.x, st.y)
                        ;

                    // let color = match cell.borrow().get_type(game.snake_length as i32) {
                    //     CellType::Empty => DARK,
                    //     CellType::Border => BROWN,
                    //     CellType::SnakeHead => GREEN,
                    //     CellType::SnakeBody => GREEN,
                    //     CellType::SnakeTail => GREEN,
                    //     CellType::Food => RED,
                    // };
                    //
                    let color = match counter % 5 {
                        0 => RED,
                        1 => YELLOW,
                        2 => GREEN,
                        3 => DARK,
                        _ => BROWN,
                    };

                    counter += 1;

                    rectangle(color, square, transform, gl);
                    // line(BLACK, 1.0, );
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let (cols, rows) = (11, 11);
    let game = Game::new(cols, rows);

    let render_settings = RenderSettings::new(
        [1000, 1000],
        game.field_size.as_array(),
    );

    // for col in game.field {
    //     for cell in col {
    //         print!("{}", match cell.cell_type {
    //             CellType::Empty => " ".to_string(),
    //             CellType::Border => "#".to_string(),
    //             CellType::SnakeHead => "@".to_string(),
    //             CellType::SnakeBody => "*".to_string(),
    //             CellType::SnakeTail => ".".to_string(),
    //             CellType::Food => "Q".to_string(),
    //         });
    //     }
    //     println!();
    // }

    /*
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V4_5;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("my-snake", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &game);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
    */

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("my-snake", render_settings.get_viewport_size())
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        rotation: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &render_settings, &game);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
