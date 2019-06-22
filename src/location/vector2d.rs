use std::ops::{Div, Mul, Add, Sub};
use super::Norm;

/**
 * A 2D vector representation.
 * The layout is packed to ensure a well behaviour in buffers.
 */
#[repr(packed)]
pub struct Vector2d<T> {
    pub x: T,
    pub y: T
}

impl<T: Copy> Vector2d<T> {
    pub fn new(x: T, y: T) -> Vector2d<T> {
        Vector2d {
            x: x,
            y: y
        }
    }

    pub fn unit(x: T, y: T) -> Vector2d<T> where T: Copy + Norm<Output = T> + Div<Output = T> {
        let v = Vector2d::new(x, y);
        v.normal()
    }

    pub fn len<D>(&self) -> D where T: Norm<Output = D> {
        T::norm(&[self.x, self.y])
    }

    pub fn normal(&self) -> Vector2d<T> where T: Copy + Norm<Output = T> + Div<Output = T> {
        let l = self.len();
        Vector2d {
            x: self.x/l,
            y: self.y/l
        }
    }

    pub fn scalar(self, other: Self) -> T where T: Mul<Output = T> + Add<Output = T> {
        self.x * other.x + self.y * other.y
    }
}

impl<T: Copy> Clone for Vector2d<T> {
    fn clone(&self) -> Vector2d<T> {
        Vector2d {
            x: self.x,
            y: self.y
        }
    }
}

impl<T: Copy> Copy for Vector2d<T> {
    // ...
}

impl<T: Copy + PartialEq> PartialEq for Vector2d<T> {
    fn eq(&self, other: &Vector2d<T>) -> bool {
        let x = self.x;
        let y = self.y;
        let other_x = other.x;
        let other_y = other.y;
        x == other_x && y == other_y
    }
}

impl<F: Sub, T: Mul<Output = F>> Mul for Vector2d<T> {
    type Output = F::Output;

    fn mul(self, other: Self) -> F::Output {
        self.x * other.y - self.y * other.x
    }
}
