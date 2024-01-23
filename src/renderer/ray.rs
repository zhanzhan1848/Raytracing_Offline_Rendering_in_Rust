use cgmath::{Vector3};

#[derive(Copy, Clone, Debug)]
pub struct Ray
{
    _origin:                    Vector3<f32>,
    _direction:                 Vector3<f32>
}

impl Ray {
    pub fn new(ori: Vector3<f32>, dir: Vector3<f32>) -> Ray
    {
        Ray
        {
            _origin: ori,
            _direction: dir
        }
    }

    pub fn origin(&self) -> Vector3<f32>
    {
        self._origin
    }

    pub fn direction(&self) -> Vector3<f32>
    {
        self._direction
    }

    pub fn at(&self, t: f32) -> Vector3<f32>
    {
        self._origin + self._direction * t
    }
}