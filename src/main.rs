extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use graphics::rectangle;
use opengl_graphics::{GlGraphics, OpenGL};
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

                let color = match cell_context.cell.get_type(game.snake_length as i32) {
                    CellType::Empty => DARK,
                    CellType::Border => BROWN,
                    CellType::SnakeHead => BLUE,
                    CellType::SnakeBody => GREEN,
                    CellType::SnakeTail => YELLOW,
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
    let game = Game::new(cols, rows);

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
    let mut app = App {
        gl: GlGraphics::new(opengl),
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
