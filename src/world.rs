extern crate rand;
use rand::random;

#[derive(Copy, Clone)]
pub struct Tile {
    pub color: [u8; 4],
}

pub struct Chunk {
    pub tiles: [[Tile; 8]; 8],
    //the outer is y the inner is x
}

impl Chunk {
    fn random() -> Chunk {
        //completely random garbage lmao
        //let new_tiles: [[Tile ; 8]; 8] = [[Tile{color:[0,0,0,0]};8]; 8]; //just make generic tiles
        //new_tiles = [[Tile{color:[random::<u8>(),random::<u8>(),random::<u8>(),random::<u8>()]};8]; 8];
        let new_tiles: [[Tile ; 8]; 8] = [[Tile{color:[0,0,0,0]};8]; 8]; //just make generic tiles
        

        Chunk {
            tiles: new_tiles,
        } //this is what is returned

    }
}

pub struct World {
    pub chunks: Vec<Vec<Chunk>>,
}

impl World {
    pub fn random(width: usize, height: usize) -> World { //broken
        
        let mut new_chunks: Vec<Vec<Chunk>> = Vec::with_capacity(height);

        for y in 0..height {
            new_chunks[y] = Vec::with_capacity(width);
            for x in 0..width {
                new_chunks[y].push(Chunk::random());
            }
        }
        
        World {
            chunks: new_chunks,
        } 
    }

    pub fn test() -> World {
        let mut new_chunks: Vec<Vec<Chunk>> = Vec::new();
        
        new_chunks.push(Vec::new());
        new_chunks[0].push(Chunk::random());

        World {
            chunks: new_chunks
        }
    }
}
