use std::ops::{Div, Mul, Add, Sub};
use super::Norm;

/**
 * A 3D vector representation.
 * The layout is packed to ensure a well behaviour in buffers.
 */
#[repr(packed)]
#[derive(Debug)]
pub struct Vector3d<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Copy> Vector3d<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3d<T> {
        Vector3d {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn normal(&self) -> Vector3d<T> where T: Copy + Norm<Output = T> + Div<Output = T> {
        let l = self.len();
        Vector3d {
            x: self.x/l,
            y: self.y/l,
            z: self.z/l
        }
    }

    pub fn len<D>(&self) -> D where T: Norm<Output = D> {
        T::norm(&[self.x, self.y, self.z])
    }

    pub fn scalar(self, other: Self) -> T where T: Mul<Output = T> + Add<Output = T> {
        self.x * other.x + self.y * other.y + self.z + other.z
    }
}

impl<T: Copy> Clone for Vector3d<T> {
    fn clone(&self) -> Vector3d<T> {
        Vector3d {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }
}

impl<T: Copy> Copy for Vector3d<T> {
    // ...
}

impl<T: Copy + PartialEq> PartialEq for Vector3d<T> {
    fn eq(&self, other: &Vector3d<T>) -> bool {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let other_x = other.x;
        let other_y = other.y;
        let other_z = other.z;
        x == other_x && y == other_y && z == other_z
    }
}

impl<T: Copy + Default> Default for Vector3d<T> {
    fn default() -> Vector3d<T> {
        Vector3d {
            x: T::default(),
            y: T::default(),
            z: T::default()
        }
    }
}

impl<T: Copy + Add> Add for Vector3d<T> {
    type Output = Vector3d<T::Output>;

    fn add(self, other: Self) -> Vector3d<T::Output> {
        Vector3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<F: Sub, T: Copy + Mul<Output = F>> Mul for Vector3d<T> where F::Output: Copy {
    type Output = Vector3d<F::Output>;

    fn mul(self, other: Self) -> Vector3d<F::Output> {
        Vector3d {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }
}

impl<T: Copy + Mul> Mul<T> for Vector3d<T> {
    type Output = Vector3d<T::Output>;

    fn mul(self, f: T) -> Vector3d<T::Output> {
        Vector3d {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f
        }
    }
}

impl<T: Copy + Div> Div<T> for Vector3d<T> {
    type Output = Vector3d<T::Output>;

    fn div(self, f: T) -> Vector3d<T::Output> {
        Vector3d {
            x: self.x / f,
            y: self.y / f,
            z: self.z / f
        }
    }
}
