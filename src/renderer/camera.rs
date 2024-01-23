use cgmath::{Vector3, Vector4, Array, InnerSpace};
use crate::renderer::custom_function::{degrees_to_radians, length, linear_to_gamma, random_double, random_in_unit_disk, random_on_hemisphere, random_unit_vector3, unit_vector3};
use crate::renderer::hittable::{HitRecord, Hittable};
use crate::renderer::hittable_list::HittableList;
use crate::renderer::interval::Interval;
use crate::renderer::material::Material;
use crate::renderer::ray::Ray;
use crate::utility::constants::{INFINITY, MAX_DEPTH, SAMPLES_PER_PIXEL};


#[derive(Copy, Clone, Debug)]
pub struct Camera
{
    // Public Parameters
    _origin:                    Vector3<f32>,
    _direction:                 Vector3<f32>,
    _aspect_ratio:              f32,
    _width:                     u32,
    _height:                    u32,
    _fov:                       f32,

    // Private Parameters
    _defocus_angle:             f32,
    _viewport_height:           f32,
    _focus_dist:                f32,
    _u:                         Vector3<f32>,
    _v:                         Vector3<f32>,
    _w:                         Vector3<f32>,
    _defocus_disk_u:            Vector3<f32>,
    _defocus_disk_v:            Vector3<f32>
}

impl Camera {
    pub fn new(ori: Vector3<f32>, dir: Vector3<f32>, ar: f32, w: u32, h: u32, fov: f32) -> Camera
    {
        // defocus
        //let focal_length = length(self.origin() - self.direction());
        let defocus_angle = 10.0;
        let focus_dist = 3.4;

        // calculation about fov
        let theta = degrees_to_radians(fov.clone());
        let in_h = (theta / 2.0).tan();

        let viewport_height = 2.0 * in_h * focus_dist;

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame
        let screen_w = unit_vector3(ori.clone() - dir.clone());
        let screen_u = unit_vector3(Vector3::new(0.0, 1.0, 0.0).cross(screen_w.clone()));
        let screen_v = screen_w.clone().cross(screen_u.clone());

        // Calculate the camera defocus disk basis vectors
        let defocus_radius = focus_dist * (degrees_to_radians(defocus_angle / 2.0)).tan();
        let defocus_disk_u = screen_u.clone() * defocus_radius;
        let defocus_disk_v = screen_v.clone() * defocus_radius;

        Camera
        {
            _origin: ori,
            _direction: dir,
            _aspect_ratio: ar,
            _width: w,
            _height: h,
            _fov: fov,

            _defocus_angle: defocus_angle,
            _viewport_height: viewport_height,
            _focus_dist: focus_dist,
            _u: screen_u,
            _v: screen_v,
            _w: screen_w,
            _defocus_disk_u: defocus_disk_u,
            _defocus_disk_v: defocus_disk_v
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

    pub fn aspect_ratio(&self) -> f32
    {
        self._aspect_ratio
    }

    pub fn width(&self) -> u32
    {
        self._width
    }

    pub fn height(&self) -> u32
    {
        self._height
    }

    pub fn fov(&self) -> f32
    {
        self._fov
    }

    pub fn render(&self, w: u32, h: u32, u: u32, v: u32, world: HittableList) -> Vector4<f32>
    {
        // Viewport
        let viewport_width = self._viewport_height * (w as f32 / h as f32);

        // UV
        let viewport_u = viewport_width * self._u.clone();
        let viewport_v = self._viewport_height * -self._v.clone();
        let delta_u = viewport_u / w as f32;
        let delta_v = viewport_v / h as f32;

        let viewport_ul = self.origin() - self._focus_dist * self._w - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_ul + 0.5 * (delta_u + delta_v);

        let mut color = Vector4::new(0.0, 0.0, 0.0, 1.0);
        for _ in 0..SAMPLES_PER_PIXEL
        {
            let r = Camera::get_ray(&self, pixel00_loc.clone(), u.clone(), delta_u.clone(), v.clone(), delta_v.clone());

            color += Camera::ray_color(r, MAX_DEPTH, world.clone());
        }

        let limit = Interval::new(0.0, 1.0);
        Vector4::new(limit.clamp(linear_to_gamma(color.x / (SAMPLES_PER_PIXEL as f32))),
                     limit.clamp(linear_to_gamma(color.y / (SAMPLES_PER_PIXEL as f32))),
                     limit.clamp(linear_to_gamma(color.z / (SAMPLES_PER_PIXEL as f32))),
                     limit.clamp(color.w / (SAMPLES_PER_PIXEL as f32)))
    }
}

impl Camera
{
    fn pixel_sample_square(delta_u: Vector3<f32>, delta_v: Vector3<f32>) -> Vector3<f32>
    {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();

        (px * delta_u) + (py * delta_v)
    }

    fn get_ray(&self, pixel00_loc: Vector3<f32>, u: u32, delta_u: Vector3<f32>, v: u32, delta_v: Vector3<f32>) -> Ray
    {
        let pixel_center = pixel00_loc + (u as f32 * delta_u) + (v as f32 * delta_v);
        let pixel_sample = pixel_center + Camera::pixel_sample_square(delta_u, delta_v);

        let mut ray_origin: Vector3<f32>;
        if self._defocus_angle <= 0.0
        {
            ray_origin = self.origin();
        }
        else
        {
            ray_origin = Camera::defocus_disk_sample(&self, self._defocus_disk_u, self._defocus_disk_v);
        }

        let ray_dir = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_dir)
    }

    fn ray_color(r: Ray, depth: u32, world: HittableList) -> Vector4<f32>
    {

        if depth <= 0
        {
            return Vector4::new(0.0, 0.0, 0.0, 1.0);
        }

        if let Some(hit) = world.hit(r, Interval::new(0.001, INFINITY))
        {
            // let direction = rec.get_normal() + random_unit_vector3();
            // return 0.1 * Camera::ray_color(Ray::new(rec.get_point(), direction), depth - 1, world);
            let mut scattered = Ray::new(Vector3::from_value(0.0), Vector3::from_value(0.0));
            let mut attenuation = Vector3::from_value(0.0);
            if hit._material.scatter(&r, &hit, &mut attenuation, &mut scattered)
            {
                let ray_color = Camera::ray_color(scattered, depth - 1, world);
                return Vector4::new(ray_color.x * attenuation.x,
                                    ray_color.y * attenuation.y,
                                    ray_color.z * attenuation.z,
                                        ray_color.w);
            }
            else
            {
                return Vector4::new(0.0, 0.0, 0.0, 1.0);
            }
        }
        let unit_dir = unit_vector3(r.direction());
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Vector4::from_value(1.0) + a * Vector4::new(0.5, 0.7, 1.0, 1.0)
    }

    fn defocus_disk_sample(&self, defocus_disk_u: Vector3<f32>, defocus_disk_v: Vector3<f32>) -> Vector3<f32>
    {
        let p = random_in_unit_disk();
        self.origin() + (p.x * defocus_disk_u) + (p.y * defocus_disk_v)
    }
}

#[cfg(test)]
mod test
{
    use cgmath::{Array, Vector3};
    use super::*;

    #[test]
    fn test()
    {
        let cam = Camera::new(Vector3::from_value(0.0), Vector3::from_value(1.0), 16.0 / 9.0, 1600, 900, 1.0);

        println!(
            "{:?}, {:?}, {}, {}, {}, {}",
            cam.origin(), cam.direction(), cam.aspect_ratio(), cam.width(), cam.height(), cam.fov()
        )
    }
}