use super::{Vector2d, Vector3d, Trigonometric, Norm, One};
use std::ops::{Mul, Div, Add, Sub};

pub struct Position<T> {
    polar: T,
    azimuth: T
}

impl<T> Position<T> {
    pub fn new(polar: T, azimuth: T) -> Position<T> {
        Position {
            polar: polar,
            azimuth: azimuth
        }
    }
}

impl<T> Position<T> where T: Copy + Trigonometric<Output = T> + Mul<Output = T> + One {
    /// Return this spherical position as a cartesian position.
    pub fn as_vector(&self, len: T) -> Vector3d<T> {
        Vector3d {
            x: len * self.polar.sin() * self.azimuth.cos(),
            y: len * self.polar.sin() * self.azimuth.sin(),
            z: len * self.polar.cos()
        }
    }

    pub fn as_unit(&self) -> Vector3d<T> {
        self.as_vector(T::ONE)
    }
}

impl<T> Position<T> where T: Copy + Trigonometric<Output = T> + One + Norm<Output = T> + Mul<Output = T> + Div<Output = T> + Sub<Output = T> + Add<Output = T> {
    /// Return the distance from 0 to PI between two points.
    pub fn distance(&self, other: &Position<T>) -> T
    {
        let a = self.as_vector(T::ONE);
        let b = other.as_vector(T::ONE);

        (a*b).len()/a.scalar(b)
    }

    /// Move in the direction of the given other point by the given factor
    /// (0 -> don't move, 1 -> move all the way, 0.5 -> between the two points, etc.).
    pub fn move_to(&self, other: &Position<T>, f: T) -> Position<T>
    {
        let angle = self.distance(other);
        let x = Vector2d::unit(T::ONE - angle.cos(), angle.sin());

        let f_angle = f * angle;
        let y = Vector2d::unit(T::ONE - f_angle.cos(), f_angle.sin());

        let k = x.scalar(y);
        let p = self.as_unit()* (T::ONE - k) + other.as_unit()*k;
        p.into()
    }

    /// Semanticaly equivalent to `move_to(other, 0.5)`, but faster.
    pub fn mean(elements: &[Position<T>]) -> Position<T> where T: Mul<usize, Output = T> {
        match elements.split_first() {
            Some((e, rest)) => {
                let mut v = e.as_unit();
                for e in rest.iter() {
                    v = v + e.as_unit()
                }
                v.into()
            },
            None => panic!("empty slice")
        }
    }
}

impl<T> From<Vector3d<T>> for Position<T> where T: Copy + Trigonometric<Output = T> + Norm<Output = T> + Div<Output = T> {
    fn from(v: Vector3d<T>) -> Position<T> {
        Position {
            polar: T::acos(v.z/v.len()),
            azimuth: T::atan(v.y/v.z)
        }
    }
}
