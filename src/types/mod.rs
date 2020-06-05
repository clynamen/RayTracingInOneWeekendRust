use nalgebra::Vector2;
use nalgebra::Vector3;
use std::fmt::*;

pub type Vector2f = Vector2<f32>;
pub type Vector3f = Vector3<f32>;
pub type Size2f = Size2<f32>;
pub type Size2i = Size2<i32>;

#[derive(Debug)]
pub struct Size2<T>
where
    T: 'static + std::cmp::PartialEq + std::clone::Clone + std::marker::Copy + std::fmt::Debug,
{
    v: Vector2<T>,
}

impl<T> Size2<T>
where
    T: 'static + std::cmp::PartialEq + std::clone::Clone + std::marker::Copy + std::fmt::Debug,
{
    pub fn new(height: T, width: T) -> Size2<T> {
        Size2 {
            v: Vector2::new(width, height),
        }
    }

    pub fn width(&self) -> T {
        self.v.x
    }

    pub fn height(&self) -> T {
        self.v.y
    }
}
