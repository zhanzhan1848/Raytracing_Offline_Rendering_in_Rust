use std::sync::Arc;
use cgmath::{Vector3, dot};
use crate::renderer::custom_function::{length_squared, set_face_normal};
use crate::renderer::hittable::{HitRecord, Hittable};
use crate::renderer::interval::Interval;
use crate::renderer::material::{Material};
use crate::renderer::ray::Ray;

pub struct Sphere
{
    _center: Vector3<f32>,
    _radius: f32,
    _material: Arc<dyn Material>,
}

impl Sphere
{
    pub fn new(p: Vector3<f32>, r: f32, material: Arc<dyn Material>) -> Self
    {
        Sphere
        {
            _center: p,
            _radius: r,
            _material: material
        }
    }
}

impl Hittable for Sphere
{
    // fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut crate::renderer::hittable::HitRecord) -> bool
    // {
    //     let oc = ray.origin() - self._center;
    //     let a = length_squared(ray.direction());
    //     let b = dot(oc, ray.direction());
    //     let c = length_squared(oc) - self._radius * self._radius;
    //
    //     let discriminant = b * b - a * c;
    //     if discriminant < 0.0
    //     {
    //         false
    //     }
    //     else
    //     {
    //         let sqrtd = discriminant.sqrt();
    //
    //         // Find the nearest root that lies in the acceptable range
    //         let mut root = (-b - sqrtd) / a;
    //         if !ray_t.surrounds(root)
    //         {
    //             root = (-b + sqrtd) / a;
    //             if !ray_t.surrounds(root)
    //             {
    //                 return false;
    //             }
    //         }
    //         rec.set_t(root);
    //         rec.set_point(ray.at(rec.get_t()));
    //         let outward_normal = (rec.get_point() - self._center.clone()) / self._radius.clone();
    //         rec.set_face_normal(ray, outward_normal);
    //         rec._material = Arc::clone(&self._material);
    //
    //         true
    //     }
    // }

    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord>
    {
        let oc = ray.origin() - self._center;
        let a = length_squared(ray.direction());
        let b = dot(oc, ray.direction());
        let c = length_squared(oc) - self._radius * self._radius;

        let discriminant = b * b - a * c;
        if discriminant < 0.0
        {
            None
        }
        else
        {
            let sqrtd = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range
            let mut root = (-b - sqrtd) / a;
            if !ray_t.surrounds(root)
            {
                root = (-b + sqrtd) / a;
                if !ray_t.surrounds(root)
                {
                    return None;
                }
            }

            let outward_normal = (ray.at(root) - self._center.clone()) / self._radius.clone();
            let (normal, front_face) = set_face_normal(ray, outward_normal);

            Some(HitRecord{ _t: root, _point: ray.at(root), _normal: normal, _material: self._material.clone(), _front_face: front_face})
        }
    }
}