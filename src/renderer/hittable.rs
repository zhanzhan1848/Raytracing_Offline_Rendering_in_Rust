use std::sync::Arc;
use cgmath::{Vector3};
use crate::renderer::interval::Interval;
use crate::renderer::material::Material;
use crate::renderer::ray::Ray;

#[derive(Clone)]
pub struct HitRecord
{
    pub _point: Vector3<f32>,
    pub _normal: Vector3<f32>,
    pub _material: Arc<dyn Material>,
    pub _t: f32,
    pub _front_face: bool
}

impl HitRecord
{
    pub fn get_point(&self) -> Vector3<f32>
    {
        self._point
    }

    pub fn get_normal(&self) -> Vector3<f32>
    {
        self._normal
    }

    pub fn get_t(&self) -> f32
    {
        self._t
    }

    pub fn set_point(&mut self, p: Vector3<f32>)
    {
        self._point = p;
    }

    pub fn set_normal(&mut self, n: Vector3<f32>)
    {
        self._normal = n;
    }

    pub fn set_t(&mut self, t: f32)
    {
        self._t = t;
    }
}

pub trait Hittable
{
    //fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord>;
}