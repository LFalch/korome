/// Struct for representating a mathematical vector: either a position, a velocity or other things similar
pub struct Vector2{
    /// Position on the x-axis
    pub x: f32,
    /// Position on the y-axis
    pub y: f32,
}

use std::ops::{Add, Sub, Mul, Div, Neg};

impl Vector2{
    /// Creates a new `Vector2` instance
    pub fn new(x: f32, y: f32) -> Self{
        Vector2{
            x: x,
            y: y,
        }
    }
    /// Creates a new unit `Vector2` in a direction in radians
    pub fn unit_vector(direction: f32) -> Self{
        let (y, x) = direction.sin_cos();
        Self::new(x, y)
    }
    /// Returns the x field
    pub fn get_x(&self) -> f32{
        self.x
    }
    /// Returns the y field
    pub fn get_y(&self) -> f32{
        self.y
    }
    /// Returns the x and y fields as a tuple
    pub fn get_x_y(&self) -> (f32, f32){
        (self.x, self.y)
    }
    /// Returns the distance betweens two vectors squared
    pub fn distance_sq(&self, other: &Self) -> f32{
        (self.x - other.x).hypot(self.y - other.y)
    }
    /// Returns the magnitude or the length of the vector
    pub fn length(&self) -> f32{
        self.x.hypot(self.y)
    }
    /// Returns the magnitude or the length of the vector squared
    pub fn length_sq(&self) -> f32{
        self.x * self.x + self.y * self.y
    }
    /// Returns direction the vector is pointing in radians
    pub fn get_direction(&self) -> f32{
        self.y.atan2(self.x)
    }
    /// Normalises the vector
    pub fn normalise(self) -> Self{
        let length = self.length();
        self / length
    }
    /// Returns direction towards an other vector
    pub fn get_direction_towards(&self, other: &Self) -> f32{
        (other.y-self.y).atan2(other.x-self.x)
    }
    /// Returns the dot product of the vectors
    pub fn dot(self, other: Self) -> f32{
        self.x * other.x + self.y * other.y
    }
    /// To float array
    pub fn to_array(self) -> [f32; 2]{
        [self.x, self.y]
    }
}

impl Add for Vector2{
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Vector2{
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vector2{
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Vector2{
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<Vector2> for f32{
    type Output = Vector2;

    fn mul(self, rhs: Vector2) -> Vector2{
        Vector2::new(self * rhs.x, self * rhs.y)
    }
}

impl Mul<f32> for Vector2{
    type Output = Vector2;

    fn mul(self, rhs: f32) -> Vector2{
        Vector2::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vector2{
    type Output = Vector2;

    fn div(self, rhs: f32) -> Vector2{
        Vector2::new(self.x/rhs, self.y/rhs)
    }
}

impl Div<Vector2> for f32{
    type Output = Vector2;

    fn div(self, rhs: Vector2) -> Vector2{
        Vector2::new(self / rhs.x, self / rhs.y)
    }
}

impl Neg for Vector2{
    type Output = Vector2;

    fn neg(self) -> Vector2{
        Vector2::new(-self.x, -self.y)
    }
}
