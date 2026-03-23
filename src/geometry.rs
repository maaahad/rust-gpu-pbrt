use num::{Float, Num, Signed};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone)]
pub struct Vector3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
where
    T: Num + Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Vector3<T>
where
    T: Signed,
{
    pub fn abs(&mut self) {
        *self = Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }
}

impl<T> Vector3<T>
where
    T: Num + Copy,
{
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: &Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

// TODO: the following method should be implemented for integer as well later
impl<T: Float> Vector3<T> {
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }
}

// Operators overloads
impl<T> Add<Vector3<T>> for Vector3<T>
where
    T: Num + Copy,
{
    type Output = Vector3<T>;
    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T> AddAssign<Vector3<T>> for Vector3<T>
where
    T: Num + Copy,
{
    fn add_assign(&mut self, rhs: Vector3<T>) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        };
    }
}

impl<T> Sub<Vector3<T>> for Vector3<T>
where
    T: Num + Copy,
{
    type Output = Vector3<T>;
    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> SubAssign<Vector3<T>> for Vector3<T>
where
    T: Num + Copy,
{
    fn sub_assign(&mut self, rhs: Vector3<T>) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T> PartialEq<Vector3<T>> for Vector3<T>
where
    T: Num + Copy,
{
    fn eq(&self, rhs: &Vector3<T>) -> bool {
        self.x == rhs.x && self.y == rhs.y && self.z == rhs.z
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Num + Copy,
    T: Mul<Output = T> + Copy,
{
    type Output = Vector3<T>;

    fn mul(self, multiplier: T) -> Self::Output {
        Self {
            x: self.x * multiplier,
            y: self.y * multiplier,
            z: self.z * multiplier,
        }
    }
}

impl<T> MulAssign<T> for Vector3<T>
where
    T: Num + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Num + Copy,
{
    type Output = Vector3<T>;
    fn div(self, rhs: T) -> Self::Output {
        // TODO assert!(rhs > 0)
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vector3<T>
where
    T: Num + Copy,
{
    fn div_assign(&mut self, rhs: T) {
        // TODO assert!(rhs > 0)
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> Neg for Vector3<T>
where
    T: Num + Neg<Output = T> + Copy,
{
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Index<usize> for Vector3<T>
where
    T: Num + Copy,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("out of bounds. Max index should be 2 for Vector3"), // TODO: any better way
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vector2<T: Num> {
    x: T,
    y: T,
}

impl<T: Num> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Operators overloads
impl<T: Num> Index<usize> for Vector2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("out of bounds. Max index should be 1 for Vector2"),
        }
    }
}
