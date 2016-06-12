/// Representation of a mathematical vector e.g. a position or velocity
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vector2<T>(pub T, pub T);

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div, Neg};
use std::convert::From;

/// Special methods for floating point vectors
pub trait FloatVector<F>{
    /// Creates a new unit vector in a specific direction
    fn unit_vector(F) -> Self;
    /// Normalises the vector
    fn normalise(self) -> Self;
    /// Returns the magnitude or the length of the vector
    fn length(&self) -> F;
    /// Returns direction the vector is pointing
    fn direction(&self) -> F;
    /// Returns direction towards another vector
    fn direction_to(&self, &Self) -> F;
    /// Returns the distance betweens two vectors
    fn distance_to(&self, &Self) -> F;
}

macro_rules! impl_for {
    ($($t:ty)*) => {$(
        impl FloatVector<$t> for Vector2<$t>{
            fn unit_vector(direction: $t) -> Self{
                let (y, x) = direction.sin_cos();
                Vector2(x, y)
            }

            fn normalise(self) -> Self{
                let length = self.length();
                self / length
            }

            fn length(&self) -> $t{
                self.0.hypot(self.1)
            }

            fn direction(&self) -> $t{
                (-self.1).atan2(self.0)
            }

            fn direction_to(&self, other: &Self) -> $t{
                (*other-*self).direction()
            }

            fn distance_to(&self, other: &Self) -> $t{
                (self.0 - other.0).hypot(self.1 - other.1)
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

impl<T> Vector2<T> {
    /// Returns the dot product of two vectors
    pub fn dot(self, other: Self) -> T where T: Mul<Output=T> + Add<Output=T>{
        self.0 * other.0 + self.1 * other.1
    }
}

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

impl<T: AddAssign> AddAssign for Vector2<T>{
    fn add_assign(&mut self, rhs: Self){
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T: SubAssign> SubAssign for Vector2<T>{
    fn sub_assign(&mut self, rhs: Self){
        self.0 -= rhs.0;
        self.1 -= rhs.1;
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

impl<T> Into<[T; 2]> for Vector2<T>{
    #[inline(always)]
    fn into(self) -> [T; 2]{
        [self.0, self.1]
    }
}

impl<T: Copy> From<[T; 2]> for Vector2<T>{
    #[inline(always)]
    fn from(array: [T; 2]) -> Self{
        Vector2(array[0], array[1])
    }
}

impl<T> Into<(T, T)> for Vector2<T>{
    #[inline(always)]
    fn into(self) -> (T, T){
        (self.0, self.1)
    }
}

impl<T> From<(T, T)> for Vector2<T>{
    #[inline(always)]
    fn from(tuple: (T, T)) -> Self{
        Vector2(tuple.0, tuple.1)
    }
}
