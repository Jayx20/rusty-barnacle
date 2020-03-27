use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

mod math;
mod world;
use world::*;
use math::*;
mod generation;

const SCREEN_WIDTH: u32 = 1920;
const SCREEN_HEIGHT: u32 = 1080;

pub struct Game {
    world: World,
    camera: Camera,
}

//graphics but also influences what chunks are loaded, so maybe rename later
#[derive(Default)]
struct Camera {
    pos: Vector2f, //could be integer I suppose, but float might be nice if zooming or something precise later on
    delta: [bool; 5], //whether key is pressed, basically. up down left right shift
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Game {
        // Load/create resources such as images here.
        Game {
            world: World::test(rand::random::<u64>()),
            camera: Default::default(),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let mut speed = 1.0;
        if self.camera.delta[4] { speed=5.0; }
        if self.camera.delta[0] { self.camera.pos.y += 500.0* as f32*speed; } //up
        if self.camera.delta[1] { self.camera.pos.y -= 500.0*args.dt as f32*speed; } //down
        if self.camera.delta[2] { self.camera.pos.x += 500.0*args.dt as f32*speed; } //left
        if self.camera.delta[3] { self.camera.pos.x -= 500.0*args.dt as f32*speed; } //right

        //self.world = World::test(rand::random());
        // use args.dt to update stuff per second, awesome
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        graphics::present(ctx)
    }
}

/*
impl Game {
    fn render(&mut self, args: &RenderArgs) {
        //use graphics::*;

        let worldref = &mut self.world; //create a handy mutable reference to the world so we can borrow it
        let camPos = self.camera.pos;
        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear([255.0/255.0, 255.0/255.0, 255.0/255.0, 1.0], gl); // Clear the screen.
            //println!("Drawing New Frame!");
            let PPU = world::PIXELS_PER_UNIT as f64;
            //graphics::rectangle(color: types::Color, rect: R, transform: math::Matrix2d, g: &mut G)
            for chunk in 0..worldref.chunks.len() { //go through each chunk
                //println!("Drawing New Chunk!");
                let chunk_xy = worldref.get_chunk_xy(chunk);
                let chunk_x = chunk_xy.x*CHUNK_WIDTH as i32; //position of the bottom left of the chunk
                let chunk_y = chunk_xy.y*CHUNK_HEIGHT as i32; //position of the bottom right of the chunk
                let chunk_width_in_pixels = CHUNK_WIDTH as i32*PIXELS_PER_UNIT as i32;
                //dont draw the chunk if it's not even visible. (assume square chunk for now)
                if
                    -camPos.y as i32+(chunk_y*PIXELS_PER_UNIT as i32) < 0-chunk_width_in_pixels                      || //above
                    -camPos.y as i32+(chunk_y*PIXELS_PER_UNIT as i32)   > SCREEN_HEIGHT as i32+chunk_width_in_pixels || //below
                    camPos.x as i32+(chunk_x*PIXELS_PER_UNIT as i32) < 0-chunk_width_in_pixels                      || //left
                    camPos.x as i32+(chunk_x*PIXELS_PER_UNIT as i32)   > SCREEN_WIDTH as i32+chunk_width_in_pixels     //right 
                {
                    continue; //skip this chunk
                }
                //TODO: MAKE DRAWING LESS SLOW!!!!

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
                    graphics::rectangle(color, [tile_x*PPU +camPos.x as f64, (SCREEN_HEIGHT as f64-tile_y*PPU) +camPos.y as f64, PPU, PPU], c.transform, gl);
                }
            }
            
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let mut speed = 1.0;
        if self.camera.delta[4] { speed=5.0; }
        if self.camera.delta[0] { self.camera.pos.y += 500.0*args.dt as f32*speed; } //up
        if self.camera.delta[1] { self.camera.pos.y -= 500.0*args.dt as f32*speed; } //down
        if self.camera.delta[2] { self.camera.pos.x += 500.0*args.dt as f32*speed; } //left
        if self.camera.delta[3] { self.camera.pos.x -= 500.0*args.dt as f32*speed; } //right
        //not sure if deltatime really helps, because piston is weird. will look into framerate stuff later. 

        //self.world = World::test(rand::random());
        // use args.dt to update stuff per second, awesome
    }
}*/

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Chunks and Stuff", "Jayx20")
		.build()
		.expect("Could not create ggez context!");

    // Create a new game and run it.
    let mut game = Game::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }

    /*
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(args) = e.update_args() {
            game.update(&args);
        }

        //TODO: move input to another file, always a good idea
        if let Some(Button::Keyboard(Key::Up)) = e.press_args() {
            game.camera.delta[0] = true;
        }
        if let Some(Button::Keyboard(Key::Down)) = e.press_args() {
            game.camera.delta[1] = true;
        }
        if let Some(Button::Keyboard(Key::Left)) = e.press_args() {
            game.camera.delta[2] = true;
        }
        if let Some(Button::Keyboard(Key::Right)) = e.press_args() {
            game.camera.delta[3] = true;
        }

        if let Some(Button::Keyboard(Key::Up)) = e.release_args() {
            game.camera.delta[0] = false;
        }
        if let Some(Button::Keyboard(Key::Down)) = e.release_args() {
            game.camera.delta[1] = false;
        }
        if let Some(Button::Keyboard(Key::Left)) = e.release_args() {
            game.camera.delta[2] = false;
        }
        if let Some(Button::Keyboard(Key::Right)) = e.release_args() {
            game.camera.delta[3] = false;
        }

        if let Some(Button::Keyboard(Key::LShift)) = e.press_args() {
            game.camera.delta[4] = true;
        }
        if let Some(Button::Keyboard(Key::LShift)) = e.release_args() {
            game.camera.delta[4] = false;
        }
        
        
    }
    */
}