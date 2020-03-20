extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod math;
mod world;
use world::*;
mod generation;


pub struct Game {
    gl: GlGraphics, // OpenGL drawing backend.
    world: World,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        //use graphics::*;

        let worldref = &mut self.world; //create a handy mutable reference to the world so we can borrow it
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear([255.0/255.0, 255.0/255.0, 255.0/255.0, 1.0], gl); // Clear the screen.
            //println!("Drawing New Frame!");
            let PPU = world::PIXELS_PER_UNIT as f64;
            //graphics::rectangle(color: types::Color, rect: R, transform: math::Matrix2d, g: &mut G)
            for chunk in 0..worldref.chunks.len() { //go through each chunk
                //println!("Drawing New Chunk!");
                let chunk_xy = worldref.get_chunk_xy(chunk);
                let chunk_x = chunk_xy.x*world::CHUNK_WIDTH as i32; //position of the bottom left of the chunk
                let chunk_y = chunk_xy.y*world::CHUNK_HEIGHT as i32; //position of the bottom right of the chunk
                
                for tile in 0..worldref.chunks[chunk].tiles.len() {
                    let tile_xy = worldref.chunks[chunk].get_tile_xy(tile);
                    let tile_x = tile_xy.x as f64 + chunk_x as f64;
                    let tile_y = tile_xy.y as f64 + chunk_y as f64;
                    let tileref = worldref.chunks[chunk].tiles[tile]; //might not actually be a ref but i dont care, it just cant be called tile lole!
                    
                    //temporary block for colors from tile type, to be moved to a differint file and use a matching function
                    let color: [f32; 4] = match tileref.tile_type {
                        Tile_Type::AIR => [0.0, 0.0, 0.0, 0.0], //transparent, i think
                        Tile_Type::DIRT => [168.0/255.0, 115.0/255.0, 72.0/255.0, 1.0],
                        Tile_Type::CLOUD => [197.0/255.0, 201.0/255.0, 201.0/255.0, 1.0],
                    };
                    //
                    graphics::rectangle(color, [tile_x*PPU, tile_y*PPU, PPU, PPU], c.transform, gl);
                }
            }
            
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        //self.world = World::test(rand::random());
        // use args.dt to update stuff per second, awesome
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Chunks and Stuff", [1920, 1080])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut game = Game {
        gl: GlGraphics::new(opengl),
        world: World::test(123456),
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