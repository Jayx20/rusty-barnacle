extern crate rand;
extern crate rand_chacha;
use rand::random;
use rand::{Rng, SeedableRng};

use std::hash::*;

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

pub struct Perlin {
    pub seed: u64,
}

impl Perlin {

    pub fn gen_chunk(&self, chunk_xy: Vector2i) -> Chunk {
        let mut new_tiles = [Tile{tile_type: Tile_Type::AIR};TILE_COUNT];
        
        let perlin_output = Perlin::gen_1d_noise(self.seed, chunk_xy, 
            Perlin_Params{
                octaves: 6, //2 to the power of this should be less than the amount of points
                scale_factor: 2.5, //half each octave
        } );

        //go through each x value
        for i in 0..CHUNK_WIDTH {
            let height = perlin_output[i] as usize;

            for h in 0..height {
                new_tiles[i + (31-h)*CHUNK_WIDTH].tile_type = Tile_Type::DIRT;
            }
        }

        let chunk: Chunk = Chunk {
            tiles: new_tiles,
        };

        chunk
    }

    //TODO: change this beacuse it makes weird bad noise - need to figure out how to take other chunks into account - but fun for my first perlin noise function
    fn gen_1d_noise(seed: u64, chunk_xy: Vector2i, params: Perlin_Params) -> [u8; CHUNK_WIDTH] {
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
        let mut output: [u8; CHUNK_WIDTH] = [0; CHUNK_WIDTH];

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

                let offset = (x-point1) / point_dist; //distance of x from the first point, divided by the distance between the two points
                let interp = lerp(random_noise[point1],random_noise[point2],1.0); //interpolate the value of x from the two points given the offset

                result += interp * amplitude; //add the interpolated value, making sure to scale to the amplitude to reduce power of higher frequency noise
            }
             let normalized = result/totalAmplitude; //gives us something between 0 and 1
             output[x] = (normalized*CHUNK_HEIGHT as f32).round() as u8 //multiply the float by the chunk height, and then round to a u8 (chunk height should be below 255)
        }   

        output
    }
    
}

//make a method that generates a chunk or at least gives you the tile for an x y spot for the chunk generator to use
//you make chunks connect by offsetting the x y by the chunk x y times 8 (chunk width), so the method gotta take x and y - smart