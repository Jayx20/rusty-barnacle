extern crate rand;
extern crate rand_chacha;
use rand::random;
extern crate ahash;
use rand::{Rng, SeedableRng};

use std::hash::*;
use std::collections::HashMap;
use ahash::AHasher;

use super::world::*;
use super::math::*;

//thanks wikipedia
fn lerp(a0: f32, a1: f32, w: f32) -> f32 {
    (1.0 - w)*a0 + w*a1
}

struct Perlin_Params {
    octaves: u32,
    scale_factor: f32,
} //parameters the perlin noise function will takes

//going to be similar to the chunk vector thing but this will hold height values just on the X axis
struct Seedmap {
    chunk_seeds: Vec<[f32; CHUNK_WIDTH]>,
    hashtable: HashMap<i32, usize>,
    seed: u64,
}

impl Seedmap {
    pub fn new(seed: u64) -> Seedmap {
        Seedmap {
            chunk_seeds: Vec::new(),
            hashtable: HashMap::new(),
            seed,
        }
    }

    pub fn get_chunk_seeds(&mut self, chunk_x: i32) -> &[f32;CHUNK_WIDTH] {
        let index: usize = match self.hashtable.get(&chunk_x) { //look and see if there is already seedmap for this chunk x
            Some(result) => *result,
            None => self.gen_seeds(chunk_x),
        };
        &self.chunk_seeds[index]
    }

    #[doc ="Generates seedmap data for a chunk X, and then returns the index of where it got added into the vector"]
    fn gen_seeds(&mut self, chunk_x: i32) -> usize {
        let mut new_seedmap: [f32; CHUNK_WIDTH] = [0.0; CHUNK_WIDTH];

        let hasher = AHasher::new_with_keys(self.seed, chunk_x as u64);
        let chunk_seed = hasher.finish(); //should hopefulyl always make the same seed for a chunk
        
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(chunk_seed);
        for i in 0..CHUNK_WIDTH {
            new_seedmap[i] = rng.gen::<f32>();
        };

        self.chunk_seeds.push(new_seedmap);
        let index: usize = self.chunk_seeds.len()-1;
        self.hashtable.insert(chunk_x, index);

        index
    }
}

pub struct Generator {
    pub seed: u64,
    seedmap: Seedmap,
}

impl Generator {

    pub fn new(seed: u64) -> Generator {
        Generator {
            seed,
            seedmap: Seedmap::new(seed),
        }
    }

    pub fn gen_chunk(&mut self, chunk_xy: Vector2i) -> Chunk {

        //"below" actually meaning above the terrain
        if chunk_xy.y < MIN_HEIGHT {
            return Chunk::fill(Tile_Type::AIR);
        }
        else if chunk_xy.y > MAX_HEIGHT {
            return Chunk::fill(Tile_Type::DIRT);
        }

        let heightmap = Generator::perlin_individual_chunk(self.seed, chunk_xy, 
            Perlin_Params{
                octaves: 6, //2 to the power of this should be less than the amount of points
                scale_factor: 2.0, //half each octave
        } );

        /*println!("Da Heights: {:?}",self.perlin_chunk(chunk_xy, 
            Perlin_Params {
                octaves: 8,
                scale_factor: 2.0,
            },
            ((MAX_HEIGHT-MIN_HEIGHT)+1)*CHUNK_HEIGHT as i32
        ));*/

        //WIP function not working yet :(
            //TODO: fix
        /*let heightmap = self.perlin_chunk(chunk_xy, 
                            Perlin_Params {
                                octaves: 7,
                                scale_factor: 1.5,
                            },
                            ((MAX_HEIGHT-MIN_HEIGHT)+1)*CHUNK_HEIGHT as i32
                        );*/

        Generator::fill_chunk_with_heights(heightmap)
    }

    fn fill_chunk_with_heights(heights: [u32; CHUNK_WIDTH]) -> Chunk {
        let mut new_tiles = [Tile{tile_type: Tile_Type::AIR};TILE_COUNT];

        //go through each x value
        for i in 0..CHUNK_WIDTH {
            let height = heights[i] as usize;

            for h in 0..height {
                new_tiles[i + (31-h)*CHUNK_WIDTH].tile_type = Tile_Type::DIRT;
            }
        }

        Chunk {
            tiles: new_tiles,
        }
    }

    //Not sure about speed because it will trash 3 chunks worth of random data - just only process 1 of the 4 chunks (or instead of 4 whatever GENERATION_WIDTH is,i just assume 4)
    //For chunks at y=0 plus the height, will generate new terrain. All other chunks will be filled with air or dirt. (for now)
    //TODO: fix, chunks not lining up and looks ugly
    fn perlin_chunk(&mut self, chunk_xy: Vector2i, params: Perlin_Params, terrain_height: i32) -> [u32; CHUNK_WIDTH] {
        let mut random_noise: [f32; CHUNK_WIDTH*GENERATION_WIDTH+1] = [0.0; CHUNK_WIDTH*GENERATION_WIDTH+1];
        //x value of first chunk in our little starting group thing
        let offset_from_group = (GENERATION_WIDTH -2)/2; //the offset in chunk x values between our chunk and the starting chunk of the generation group
        let starting_chunk = (chunk_xy.x / 2)*2 - offset_from_group as i32; //integer math rounds it to the first multiple of 2 then move back until at the start of our little group
        //chunks to load seeded data from will be the starting chunk plus the rest until GENERATION_WIDTH is met
        let mut index = 0;
        for x in starting_chunk..starting_chunk+GENERATION_WIDTH as i32 {
            let seedsref: &[f32; CHUNK_WIDTH] = self.seedmap.get_chunk_seeds(x);
            for subindex in 0..seedsref.len() {
                random_noise[index*CHUNK_WIDTH + subindex] = seedsref[subindex];
            }
            index += 1;
        } //this loop loads all of the data for the chunks into the random_noise array
        random_noise[CHUNK_WIDTH*GENERATION_WIDTH] = random_noise[0]; //sets the last value to the first as that is needed by perlin noise
        
        //time to use perlin noise to make the random noise cool
        let mut output: [u32; CHUNK_WIDTH] = [0; CHUNK_WIDTH];

        for x in CHUNK_WIDTH*offset_from_group..CHUNK_WIDTH*(offset_from_group+1) { //go through each X value starting at our chunk and determine Y value from that
            let mut result: f32 = 0.0;
            let mut amplitude: f32 = 1.0;
            let mut totalAmplitude: f32 = 0.0; //starting values

            for octave in 0..params.octaves {
                amplitude = amplitude / params.scale_factor; //lower the importance of high frequency noise
                totalAmplitude += amplitude; //add to the total for normalization later

                let point_dist = CHUNK_WIDTH*GENERATION_WIDTH / 2usize.pow(octave); //divide by 2 to the power of whatever - hopefully width will be divisible by powers of 2
                //as example (with current setup), on the first octave the point_dist will be 32, then 16, then 8, then 4, then 2, then 1 at the end

                //find the two points that we will interpolate from - beginning octaves will find distance points and as we grow towards 'higher frequencies' the points grow closer to our X value
                let point1: usize = (x / point_dist) * point_dist; //index of first point - division finds the first point before x
                let point2: usize = (point1 + point_dist) % CHUNK_WIDTH*GENERATION_WIDTH; //second point will be first point plus the dist

                let offset = (x-point1) as f32 / point_dist as f32; //distance of x from the first point, divided by the distance between the two points
                let interp = lerp(random_noise[point1],random_noise[point2],offset); //interpolate the value of x from the two points given the offset

                result += interp * amplitude; //add the interpolated value, making sure to scale to the amplitude to reduce power of higher frequency noise
                //println!("Debug Data for {}. Amplitude: {}, Point_Dist: {}, Points 1 and 2, {}-{}, Their values: {}, {}, Offset: {}, Interp: {}, Result: {}", x, amplitude, point_dist, point1,point2, random_noise[point1], random_noise[point2], offset, interp, result);
            }
             let normalized = result/totalAmplitude; //gives us something between 0 and 1
             
             output[x-CHUNK_WIDTH*offset_from_group] = (normalized*terrain_height as f32).round() as u32 //multiply the float by the terrain height and round to integer so we get a value that fits
        }   

        output
    }

    // only makes individual chunks that dont connect
    fn perlin_individual_chunk(seed: u64, chunk_xy: Vector2i, params: Perlin_Params) -> [u32; CHUNK_WIDTH] {
        let mut hasher = SipHasher::new();
        hasher.write_u64(seed);
        hasher.write_u32(chunk_xy.x as u32);
        hasher.write_u32(chunk_xy.y as u32);
        let chunk_seed = hasher.finish(); //should be a random seed for the chunk itself
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(chunk_seed);
        let mut random_noise: [f32; CHUNK_WIDTH+1] = [0.0; CHUNK_WIDTH+1]; //leave one extra element on the end
        for i in 0..CHUNK_WIDTH {
            random_noise[i] = rng.gen::<f32>(); //set random values for all values except the last
        } //the extra element has to be equal to the first element, but don't generate something random for it
        random_noise[CHUNK_WIDTH] = random_noise[0];

        //time to use perlin noise to make the random noise cool
        let mut output: [u32; CHUNK_WIDTH] = [0; CHUNK_WIDTH];

        for x in 0..CHUNK_WIDTH { //go through each X value and determine Y value
            let mut result: f32 = 0.0;
            let mut amplitude: f32 = 1.0;
            let mut totalAmplitude: f32 = 0.0; //starting values

            for octave in 0..params.octaves {
                amplitude = amplitude / params.scale_factor; //lower the importance of high frequency noise
                totalAmplitude += amplitude; //add to the total for normalization later

                let point_dist = CHUNK_WIDTH / 2usize.pow(octave); //divide by 2 to the power of whatever - hopefully width will be divisible by powers of 2
                //as example (with current setup), on the first octave the point_dist will be 32, then 16, then 8, then 4, then 2, then 1 at the end

                //find the two points that we will interpolate from - beginning octaves will find distance points and as we grow towards 'higher frequencies' the points grow closer to our X value
                let point1: usize = (x / point_dist) * point_dist; //index of first point - division finds the first point before x
                let point2: usize = (point1 + point_dist) % CHUNK_WIDTH; //second point will be first point plus the dist

                let offset = (x-point1) as f32 / point_dist as f32; //distance of x from the first point, divided by the distance between the two points
                let interp = lerp(random_noise[point1],random_noise[point2],offset); //interpolate the value of x from the two points given the offset

                result += interp * amplitude; //add the interpolated value, making sure to scale to the amplitude to reduce power of higher frequency noise
                //println!("{:?}",(offset, interp));
            }
             let normalized = result/totalAmplitude; //gives us something between 0 and 1
             
             output[x] = (normalized*CHUNK_HEIGHT as f32).round() as u32 //multiply the float by the chunk height, and then round to a u8 (chunk height should be below 255)
        }   

        output
    }
    
}

//TODO:
//use heightmap thing
//make a method that generates a chunk or at least gives you the tile for an x y spot for the chunk generator to use
//you make chunks connect by offsetting the x y by the chunk x y times 8 (chunk width), so the method gotta take x and y - smart