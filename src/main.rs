extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::ops::Deref;
use glutin_window::GlutinWindow as Window;
use graphics::rectangle;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::Button::Keyboard;
use piston::{ButtonEvent, Key};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use my_snake::*;


struct App {
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs, render_settings: &RenderSettings, game: &Game) {
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
            clear(GRAY, gl);


            for cell_context in game.cell_iter() {

                let transform = c.transform.trans(
                    cell_context.cell_position.x * render_settings.square_size.x,
                    cell_context.cell_position.y * render_settings.square_size.y,
                );

                let color = match cell_context.cell.borrow().cell_type {
                    CellType::Empty => DARK,
                    CellType::Border => BROWN,
                    CellType::Snake(snake_body_part) => match snake_body_part {
                        SnakeBodyPart::Head(_) => GREEN,
                        SnakeBodyPart::Body(_) => YELLOW,
                        SnakeBodyPart::Tail(_) => GREEN_DARK,
                    },
                    CellType::Food => RED,
                };

                rectangle(color, render_settings.square, transform, gl);
                line(BLACK, 1.0, [0., 0., render_settings.square_size.x, 0.], transform, gl);
                line(BLACK, 1.0, [0., 0., 0., render_settings.square_size.y], transform, gl);
            }

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
    }
}

fn main() {
    let (cols, rows) = (11, 11);
    let mut game = match Game::new(cols, rows) {
        Ok(game) => game,
        Err(e) => {
            eprintln!("Failed to initialize game: {}", e);
            std::process::exit(1);
        },
    };

    let render_settings = RenderSettings::new(
        [100, 100],
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
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    let mut dt: f64 = 0.0;

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
            app.update(&args);
            dt += args.dt;
        }
        
        if dt > 0.5 {
            game.update_game_state();
            dt = 0.0;
        }

        if let Some(args) = e.render_args() {
            app.render(&args, &render_settings, &game);
        }
    }
}
