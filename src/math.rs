//I could use a math library, but it's more fun and educational to write my own stuff. I get to learn operator overloading in Rust!
use std::ops;

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector2f {
    pub x: f32,
    pub y: f32,
}
//multiplying by a scalar
impl ops::Mul<f32> for Vector2f {
    type Output = Vector2f;

    fn mul(self, rhs: f32) -> Vector2f {
        Vector2f {
            x: self.x*rhs,
            y: self.x*rhs,
        }
    }
}
impl ops::MulAssign<f32> for Vector2f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x*=rhs;
        self.y*=rhs;
        
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}