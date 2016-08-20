use ::num_traits::Float;

/// Representation of a mathematical vector e.g. a position or velocity
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[deprecated(since = "0.12.4", note = "use the `simple-vector2d` crate instead")]
pub struct Vector2<T>(pub T, pub T);

use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Neg};
use std::convert::From;

impl<T: Float> Vector2<T>{
    /// Creates a new unit vector in a specific direction
    pub fn unit_vector(direction: T) -> Self{
        let (y, x) = direction.sin_cos();
        Vector2(x, y)
    }
    /// Normalises the vector
    pub fn normalise(self) -> Self{
        self / self.length()
    }
    /// Returns the magnitude or the length of the vector
    pub fn length(self) -> T{
        self.0.hypot(self.1)
    }
    /// Returns direction the vector is pointing
    pub fn direction(self) -> T{
        self.1.atan2(self.0)
    }
    /// Returns direction towards another vector
    pub fn direction_to(self, other: Self) -> T{
        (other-self).direction()
    }
    /// Returns the distance betweens two vectors
    pub fn distance_to(self, other: Self) -> T{
        (self-other).length()
    }
    /// Returns `true` if either component is `NaN`.
    pub fn is_nan(&self) -> bool{
        self.0.is_nan() || self.1.is_nan()
    }
    /// Returns `true` if either component is positive or negative infinity.
    pub fn is_infinite(&self) -> bool{
        self.0.is_infinite() || self.1.is_infinite()
    }
    /// Returns `true` if either component is neither infinite nor `NaN`.
    pub fn is_finite(&self) -> bool{
        self.0.is_finite() || self.1.is_finite()
    }
    /// Returns `true` if either component is neither zero, inifnite, subnormal nor `NaN`.
    pub fn is_normal(&self) -> bool{
        self.0.is_normal() || self.1.is_normal()
    }
}

macro_rules! impl_for {
    ($($t:ty)*) => {$(
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
}impl_for!{f32 f64}

impl<T> Vector2<T> {
    /// Returns the dot product of two vectors
    pub fn dot(self, other: Self) -> <<T as Mul>::Output as Add>::Output
    where T: Mul, <T as Mul>::Output: Add{
        self.0 * other.0 + self.1 * other.1
    }
}

impl<T: Add> Add for Vector2<T>{
    type Output = Vector2<T::Output>;

    fn add(self, rhs: Self) -> Self::Output{
        Vector2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T: Sub> Sub for Vector2<T>{
    type Output = Vector2<T::Output>;

    fn sub(self, rhs: Self) -> Self::Output{
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

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T>{
    fn mul_assign(&mut self, rhs: T){
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Vector2<T>{
    fn div_assign(&mut self, rhs: T){
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

impl<T: Mul + Copy> Mul<T> for Vector2<T>{
    type Output = Vector2<T::Output>;

    fn mul(self, rhs: T) -> Self::Output{
        Vector2(self.0 * rhs, self.1 * rhs)
    }
}

impl<T: Div + Copy> Div<T> for Vector2<T>{
    type Output = Vector2<T::Output>;

    fn div(self, rhs: T) -> Self::Output{
        Vector2(self.0/rhs, self.1/rhs)
    }
}

impl<T: Neg> Neg for Vector2<T>{
    type Output = Vector2<T::Output>;

    fn neg(self) -> Self::Output{
        Vector2(-self.0, -self.1)
    }
}

impl<T> Into<[T; 2]> for Vector2<T>{
    #[inline]
    fn into(self) -> [T; 2]{
        [self.0, self.1]
    }
}

impl<T: Copy> From<[T; 2]> for Vector2<T>{
    #[inline]
    fn from(array: [T; 2]) -> Self{
        Vector2(array[0], array[1])
    }
}

impl<T> Into<(T, T)> for Vector2<T>{
    #[inline]
    fn into(self) -> (T, T){
        (self.0, self.1)
    }
}

impl<T> From<(T, T)> for Vector2<T>{
    #[inline]
    fn from(tuple: (T, T)) -> Self{
        Vector2(tuple.0, tuple.1)
    }
}
