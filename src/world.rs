extern crate rand;
use rand::{thread_rng, Rng};
use std::collections::HashMap;

pub const PIXELS_PER_UNIT: u32 = 8; //how many pixels on the screen per every unit in the game, might not need to be constant (consider zooming etc)
pub const CHUNK_WIDTH:  usize = 32;
pub const CHUNK_HEIGHT: usize = 32;

pub const TILE_COUNT: usize = CHUNK_WIDTH*CHUNK_HEIGHT;

pub const MIN_HEIGHT: i32 = 2;
pub const MAX_HEIGHT: i32 = 2; //generate cool chunks between 0 and 2
pub const GENERATION_WIDTH: usize = 4; //how many chunks wide should each generation group be - wider means larger clumps of chunks are generated together - laggier but smoother = not actually sure though
//MAKE GENERATION WIDTH AN EVEN NUMBER

use super::generation;
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

    #[doc="Makes a new chunk filled with a certain type of tile."]
    pub fn fill(tile_type: Tile_Type) -> Chunk {
        Chunk {
            tiles: [Tile{tile_type};TILE_COUNT],
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
    generator: generation::Generator,
    //noise: time to write my own noise function/struct
}

impl World {
    pub fn add_chunk(&mut self, chunk: Chunk, xy: Vector2i) {
        self.chunks.push(chunk); //add the chunk to the vector array
        let index: usize = self.chunks.len()-1; //should be where the chunk got put
        self.chunkmap.insert(index, xy); //associates that chunk with the X and Y coordinates
    }

    //pub fn del_chunk would be nice

    pub fn test(seed: u64) -> World {
        let mut world : World = World {
            chunks: Vec::new(),
            chunkmap: HashMap::new(),
            generator: generation::Generator::new(seed),
        };
        //Makes a fancy square of 4 chunks
        //let chunk = world.generator.gen_chunk(Vector2i{x:1, y:2});
        //world.add_chunk(chunk, Vector2i{x:1,y:2});
        
        
        //Temporary nicer looking generation
        for x in 0..10 {
            for y in 0..5 {
                let chunk = world.generator.gen_chunk(Vector2i{x,y});
                world.add_chunk(chunk, Vector2i{x,y});
            }
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
