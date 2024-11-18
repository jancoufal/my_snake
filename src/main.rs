extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::math::Scalar;
use graphics::rectangle;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::Button::Keyboard;
use piston::{ButtonEvent, Key};

use my_snake::*;
use my_snake::game_state::GameStateX;
use my_snake::point_2d::Point2D;

struct App {
    gl: GlGraphics,
    t: f64,
    dt: f64,
    initial_update_game_state_cooldown: f64,
    update_game_state_cooldown: f64,
}

impl App {
    fn new(gl: GlGraphics, initial_update_game_state_cooldown: f64) -> App {
        App {
            gl,
            t: 0.0,
            dt: 0.0,
            initial_update_game_state_cooldown,
            update_game_state_cooldown: initial_update_game_state_cooldown
        }
    }
    
    fn render(&mut self, args: &RenderArgs, render_settings: &RenderSettings, game: &mut Game, texture_grass: &Texture) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        const BROWN: [f32; 4] = [1.0, 0.5, 0.2, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const GREEN_DARK: [f32; 4] = [0.0, 0.75, 0.0, 1.0];
        const DARK: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
        const GRAY: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.2, 0.0, 1.0, 1.0];
        const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
        const MAGENTA: [f32; 4] = [1.0, 0.0, 0.9, 1.0];

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            Image::new().draw(
                texture_grass,
                &c.draw_state,
                c.transform,
                gl
            );

            rectangle(
                RED,
                render_settings.square,
                c.transform
                    .trans(
                    render_settings.viewport_size.x as f64 / 2.,
                    render_settings.viewport_size.y as f64 / 2.
                    )
                    .rot_rad(
                        self.t as Scalar / std::f64::consts::PI,
                    )
                    .trans(
                    200.0 * f64::sin(self.t),
                    200.0 * f64::cos(self.t)
                ),
                gl
            );

            // println!("frame...");
            for cell_context in game.cell_iter_mut() {
                let mut cell = cell_context.cell.borrow_mut();
                let transform = c.transform.trans(
                    cell_context.cell_position.x * render_settings.square_size.x,
                    cell_context.cell_position.y * render_settings.square_size.y,
                );

                let color = match cell.cell_type {
                    CellType::Uninitialized => MAGENTA,
                    CellType::Empty => DARK,
                    CellType::Border => BROWN,
                    CellType::Snake(snake_body_part) => match snake_body_part {
                        SnakeBodyPart::Head(_) => GREEN,
                        SnakeBodyPart::Body(_) => YELLOW,
                        SnakeBodyPart::Tail(_) => GREEN_DARK,
                    },
                    CellType::Food => RED,
                };

                if color != DARK {
                    rectangle(color, render_settings.square, transform, gl);
                }
            }
        });
    }
    
    fn start(&mut self) {
        self.update_game_state_cooldown = self.initial_update_game_state_cooldown;
    }

    fn update(&mut self, args: &UpdateArgs, game: &mut Game) {
        self.t += args.dt;
        self.dt = args.dt;
        
        self.update_game_state_cooldown -= self.dt;
        if self.update_game_state_cooldown <= 0.0 {
            game.update_game_state();
            self.update_game_state_cooldown = self.initial_update_game_state_cooldown;
        }
    }
}

fn main() {

    let board_size = Point2D::new(3, 3);
    
    let game_state = match GameStateX::new(board_size) {
        Ok(game_state) => game_state,
        Err(e) => {
            eprintln!("Failed to initialize game: {}", e);
            std::process::exit(1);
        }
    };
    
    panic!("END!");

    let (cols, rows) = (12, 12);
    let mut game = match Game::new(cols, rows) {
        Ok(game) => game,
        Err(e) => {
            eprintln!("Failed to initialize game: {}", e);
            std::process::exit(1);
        },
    };

    let render_settings = RenderSettings::new(
        [1000, 1000],
        game.field_size.as_array(),
    );

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("my-snake", render_settings.get_viewport_size())
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(
        GlGraphics::new(opengl),
        0.5
    );

    let mut events = Events::new(EventSettings::new());

    let tex_grass = Texture::from_path(
        "/Users/jcoufal/dev/rust/my-snake/assets/grass-1024.jpg", // todo: relative path
        &TextureSettings::new(),
    ).expect("Could not load grass texture");

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.button_args() {
            match args.button {
                Keyboard(key) => match key {
                    Key::Up => game.set_movement_direction(Direction::Up),
                    Key::Down => game.set_movement_direction(Direction::Down),
                    Key::Left => game.set_movement_direction(Direction::Left),
                    Key::Right => game.set_movement_direction(Direction::Right),
                    _ => {},
                },
                _ => {},
            }
            println!("{:?}", args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args, &mut game);
        }
        
        if let Some(args) = e.render_args() {
            app.render(&args, &render_settings, &mut game, &tex_grass);
        }
    }
}
