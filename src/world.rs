extern crate rand;
use rand::random;
use std::collections::HashMap;

pub static PIXELS_PER_UNIT: u32 = 32; //how many pixels on the screen per every unit in the game, might not need to be constant (consider zooming etc)
pub static CHUNK_WIDTH:  usize = 8;
pub static CHUNK_HEIGHT: usize = 8;

//if needed will replace Points with Vector2f and Vector2i from a generic vector library or i'll write one myself
#[derive(Copy, Clone, Debug)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}
#[derive(Copy, Clone, Debug)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

fn random_color() -> f32 {
    (random::<u8>() as f32) /255.0
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub color: [f32; 4],
    //going to change so tiles can be cooler but idk what to do yet, maybe just store an id?
}

pub struct Chunk {
    pub tiles: [Tile; 64],
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
}

impl Chunk {
    fn random() -> Chunk {
        let mut new_tiles: [Tile; 64] = [Tile{color:[0.2,0.0,0.0,1.0]};64]; //just make generic tiles
        
        for tile in new_tiles.iter_mut() {
            *tile = Tile {color: [random_color(), random_color(), random_color(), 1.0]};
        } //replace each tile with a new random one

        Chunk {
            tiles: new_tiles,
        } //this is what is returned

    }
}

pub struct World {
    pub chunks: Vec<Chunk>,
    chunkmap: HashMap<usize, Vector2i>,
}

impl World {
    pub fn add_chunk(&mut self, chunk: Chunk, xy: Vector2i) {
        self.chunks.push(chunk); //add the chunk to the vector array
        let index: usize = self.chunks.len()-1; //should be where the chunk got put
        self.chunkmap.insert(index, xy); //associates that chunk with the X and Y coordinates
    }

    //pub fn del_chunk would be nice

    pub fn test() -> World {
        let mut world : World = World {
            chunks: Vec::new(),
            chunkmap: HashMap::new(),
        };

        //Makes a fancy square of 4 chunks
        world.add_chunk(Chunk::random(), Vector2i{x:0,y:0});
        world.add_chunk(Chunk::random(), Vector2i{x:0,y:1});
        world.add_chunk(Chunk::random(), Vector2i{x:1,y:0});
        world.add_chunk(Chunk::random(), Vector2i{x:1,y:1});

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
