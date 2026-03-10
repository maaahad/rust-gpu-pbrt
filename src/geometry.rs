use std::ops::{Add, Index};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

// TODO: implement other pub method
impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

// Operators overloads
impl<T: Add<Output = T>> Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;
    fn add(self, other: Vector3<T>) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: PartialEq> PartialEq<Vector3<T>> for Vector3<T> {
    fn eq(&self, other: &Vector3<T>) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl<T> Index<usize> for Vector3<T> {
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

#[derive(Debug)]
pub struct Vector2<T> {
    x: T,
    y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Operators overloads
impl<T> Index<usize> for Vector2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("out of bounds. Max index should be 1 for Vector2"),
        }
    }
}
