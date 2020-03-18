extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod world;
use world::World;

pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    world: World,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        //use graphics::*;

        for chunk in self.world.chunks.iter_mut().flatten() {
            self.gl.draw(args.viewport(), |c, gl| {
                for tile in chunk.tiles.iter().flatten() {
                    //i have yet to make this work, lole!
                    //TODO: this is what must be made to work
                }
            });
        }

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            //clear(GREEN, gl);
            //graphics::rectangle(color: types::Color, rect: R, transform: math::Matrix2d, g: &mut G)
            // i dont understand this man
            
            //time for fun code
            

        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // use args.dt to update stuff per second, awesome
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        world: World::test(),
    };


    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }
    }
}