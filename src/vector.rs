/// Struct for representating a mathematical vector e.g. a position or velocity
#[derive(Copy, Clone, Debug)]
pub struct Vector2<T>(pub T, pub T);

use std::ops::{Add, Sub, Mul, Div, Neg};
use std::convert::From;

macro_rules! impl_for {
    ($($t:ty)*) => {$(
        impl Vector2<$t>{
            /// Creates a new unit `Vector2` in a direction in radians
            pub fn unit_vector(direction: $t) -> Self{
                let (y, x) = direction.sin_cos();
                Vector2(x, y)
            }
            /// Returns the x field
            pub fn get_x(&self) -> $t{
                self.0
            }
            /// Returns the y field
            pub fn get_y(&self) -> $t{
                self.1
            }
            /// Returns the x and y fields as a tuple
            pub fn get_x_y(&self) -> ($t, $t){
                (self.0, self.1)
            }
            /// Returns the distance betweens two vectors squared
            pub fn distance_sq(&self, other: &Self) -> $t{
                (self.0 - other.0).hypot(self.1 - other.1)
            }
            /// Returns the magnitude or the length of the vector
            pub fn length(&self) -> $t{
                self.0.hypot(self.1)
            }
            /// Returns the magnitude or the length of the vector squared
            pub fn length_sq(&self) -> $t{
                self.0 * self.0 + self.1 * self.1
            }
            /// Returns direction the vector is pointing in radians
            pub fn get_direction(&self) -> $t{
                self.1.atan2(self.0)
            }
            /// Normalises the vector
            pub fn normalise(self) -> Self{
                let length = self.length();
                self / length
            }
            /// Returns direction towards an other vector
            pub fn get_direction_towards(&self, other: &Self) -> $t{
                (other.1-self.1).atan2(other.0-self.0)
            }
            /// Returns the dot product of the vectors
            pub fn dot(self, other: Self) -> $t{
                self.0 * other.0 + self.1 * other.1
            }
            /// To float array
            pub fn to_array(self) -> [$t; 2]{
                [self.0, self.1]
            }
        }

        impl Mul<Vector2<$t>> for $t{
            type Output = Vector2<$t>;

            fn mul(self, rhs: Vector2<$t>) -> Vector2<$t>{
                Vector2(self * rhs.0, self * rhs.1)
            }
        }

        impl Div<Vector2<$t>> for $t{
            type Output = Vector2<$t>;

            fn div(self, rhs: Vector2<$t>) -> Vector2<$t>{
                Vector2(self / rhs.0, self / rhs.1)
            }
        }
    )*};
}

impl_for!(f32 f64);

impl<T: Add<Output=T>> Add for Vector2<T>{
    type Output = Self;

    fn add(self, rhs: Self) -> Self{
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub<Output=T>> Sub for Vector2<T>{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self{
        Vector2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T: Mul<Output=T> + Copy> Mul<T> for Vector2<T>{
    type Output = Self;

    fn mul(self, rhs: T) -> Self{
        Vector2(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Div<Output=T> + Copy> Div<T> for Vector2<T>{
    type Output = Self;

    fn div(self, rhs: T) -> Self{
        Vector2(self.0/rhs, self.1/rhs)
    }
}

impl<T: Neg<Output=T>> Neg for Vector2<T>{
    type Output = Self;

    fn neg(self) -> Self{
        Vector2(-self.0, -self.1)
    }
}

impl<T> From<(T, T)> for Vector2<T>{
    fn from(tuple: (T, T)) -> Self{
        Vector2(tuple.0, tuple.1)
    }
}
