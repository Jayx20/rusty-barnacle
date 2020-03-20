extern crate rand;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub const PIXELS_PER_UNIT: u32 = 8; //how many pixels on the screen per every unit in the game, might not need to be constant (consider zooming etc)
pub const CHUNK_WIDTH:  usize = 32;
pub const CHUNK_HEIGHT: usize = 32;

pub const TILE_COUNT: usize = CHUNK_WIDTH*CHUNK_HEIGHT;

pub const SEED: u64 = 12345;

use super::noise;
use super::math::*;

#[derive(Copy, Clone)]
pub enum Tile_Type {
    AIR = 0,
    DIRT = 1,
    CLOUD = 2,
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub tile_type: Tile_Type,
}

pub struct Chunk {
    pub tiles: [Tile; TILE_COUNT],
    //8 by 8 square
}

impl Chunk {
    #[doc = "Allows you to select a tile given relative X and Y coordinates."]
    fn get_tile(&mut self, x: usize, y: usize) -> &mut Tile {
        &mut self.tiles[x + y*CHUNK_WIDTH] //returns mutable reference
    }

    #[doc = "Returns the X and Y coordinates of a tile relative to the chunk given the index."]
    pub fn get_tile_xy(&self, index: usize) -> Vector2f {
        let y: f32 = (index/CHUNK_WIDTH) as f32;
        let x: f32 = (index%CHUNK_WIDTH) as f32;
        Vector2f {
            x,
            y,
        }
    }
    
    pub fn new() -> Chunk {
        Chunk {
            tiles: [Tile{tile_type: Tile_Type::AIR};TILE_COUNT],
        }
    }

    fn random() -> Chunk {
        let mut new_tiles: [Tile; TILE_COUNT] = [Tile{tile_type: Tile_Type::AIR};TILE_COUNT]; //just make generic tiles
        
        let mut rng = rand::thread_rng();

        for tile in new_tiles.iter_mut() {
            let rnd: u8 = rng.gen_range(0,3);
            let tile_type: Tile_Type;
            tile_type = match rnd {
                0 => Tile_Type::AIR,
                1 => Tile_Type::DIRT,
                2 => Tile_Type::CLOUD,
                _ => panic!("How do you random number???"),
            };
            *tile = Tile {tile_type};
        } //replace each tile with a new random one

        Chunk {
            tiles: new_tiles,
        } //this is what is returned

    }
}

pub struct World {
    pub chunks: Vec<Chunk>,
    chunkmap: HashMap<usize, Vector2i>,
    noise_gen: noise::Perlin,
    //noise: time to write my own noise function/struct
}

impl World {
    pub fn add_chunk(&mut self, chunk: Chunk, xy: Vector2i) {
        self.chunks.push(chunk); //add the chunk to the vector array
        let index: usize = self.chunks.len()-1; //should be where the chunk got put
        self.chunkmap.insert(index, xy); //associates that chunk with the X and Y coordinates
    }

    pub fn gen_chunk(&mut self, xy: Vector2i) {
        let mut new_chunk: Chunk = Chunk::new();
        //TODO: noise stuff
        self.add_chunk(new_chunk, xy);
    }

    //pub fn del_chunk would be nice

    pub fn test() -> World {
        let mut world : World = World {
            chunks: Vec::new(),
            chunkmap: HashMap::new(),
            noise_gen: noise::Perlin {seed:SEED},
        };
        //world.noise_gen.debug_print_noise();

        //Makes a fancy square of 4 chunks
        //world.add_chunk(world.noise_gen.gen_chunk(Vector2i{x:0,y:3}), Vector2i{x:0,y:3});
        
        //world.add_chunk(Chunk::random(), Vector2i{x:1,y:1});

        for i in 0..10 {
            world.add_chunk(world.noise_gen.gen_chunk(Vector2i{x:i,y:3}), Vector2i{x:i,y:3});
        }

        world
    }

    pub fn get_chunk_xy(&self, index: usize) -> Vector2i {
        match self.chunkmap.get(&index) {
            Some(xy) => return *xy,
            None => panic!("Requested xy of a chunk that does not exist. (index: {})", index), //this would mean a chunk that didn't exist was called. This shouldn't ever happen if the program is working.
            //Funny Note: the first time I tried to compile this program and run it after adding a lot of stuff, the thing above I thought would never happen happened immediately.
        }
    }
    
}
