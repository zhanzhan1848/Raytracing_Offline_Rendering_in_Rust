use std::rc::Rc;
use std::sync::Arc;
use cgmath::{Array, Vector3, Vector4};
use crate::renderer::hittable_list::HittableList;
use super::camera;
use crate::renderer::sphere::Sphere;
use crate::renderer::material::{Dielectric, Lambertian, Metal};
use crate::utility::constants::PI;

pub fn render(w: u32, h: u32, u: u32, v: u32) -> Vector4<f32>
 {
     // Camera
     let cam = camera::Camera::new(Vector3::new(-2.0, 2.0, 1.0),
                                   Vector3::new(0.0, 0.0, -1.0), 16.0 / 9.0,
                                    w, h, 20.0);

     // Metarial
     let material_ground = Lambertian::new(Vector3::new(0.8, 0.8, 0.0));
     let material_center = Lambertian::new(Vector3::new(0.1, 0.2, 0.5));
     let material_left = Dielectric::new(1.5);
     let material_left2 = Dielectric::new(1.5);
     let material_right = Metal::new(Vector3::new(0.8, 0.6, 0.2), 0.0);

     // World
     let mut world = HittableList::new();
     world.add(Rc::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5, Arc::new(material_center))));
     world.add(Rc::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0, Arc::new(material_ground))));
     world.add(Rc::new(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5, Arc::new(material_left))));
     world.add(Rc::new(Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.4, Arc::new(material_left2))));
     world.add(Rc::new(Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5, Arc::new(material_right))));

     cam.render(w, h, u, v, world)
 }