use cgmath::{dot, Vector3};
use num::abs;
use rand::Rng;
use crate::renderer::ray::Ray;
use crate::utility::constants::PI;


#[inline]
pub fn length_squared(v: Vector3<f32>) -> f32
{
    v.x * v.x + v.y * v.y + v.z * v.z
}

#[inline]
pub fn length(v: Vector3<f32>) -> f32
{
    length_squared(v).sqrt()
}

#[inline]
pub fn unit_vector3(v: Vector3<f32>) -> Vector3<f32>
{
    v / length(v)
}

#[inline]
pub fn random_double() -> f32
{
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..1.0)
}

#[inline]
pub fn random_double_range(min: f32, max: f32) -> f32
{
    min + (max - min) * random_double()
}

#[inline]
pub fn random_vector3() -> Vector3<f32>
{
    Vector3::new(random_double(), random_double(), random_double())
}

#[inline]
pub fn random_vector3_range(min: f32, max: f32) -> Vector3<f32>
{
    Vector3::new(random_double_range(min, max),
                 random_double_range(min, max),
                 random_double_range(min, max))
}

#[inline]
pub fn random_in_unit_sphere() -> Vector3<f32>
{
    loop
    {
        let p = random_vector3_range(-1.0, 1.0);
        if length_squared(p.clone()) < 1.0
        {
            return p;
        }
    }
}

#[inline]
pub fn random_in_unit_disk() -> Vector3<f32>
{
    loop
    {
        let p = Vector3::new(random_double_range(-1.0, 1.0), random_double_range(-1.0, 1.0), 0.0);
        if length_squared(p.clone()) < 1.0
        {
            return p;
        }
    }
}

#[inline]
pub fn random_unit_vector3() -> Vector3<f32>
{
    unit_vector3(random_in_unit_sphere())
}

#[inline]
pub fn random_on_hemisphere(normal: Vector3<f32>) -> Vector3<f32>
{
    let on_unit_sphere = random_unit_vector3();
    if dot(on_unit_sphere.clone(), normal) > 0.0
    {
        on_unit_sphere
    }
    else
    {
        -on_unit_sphere
    }
}
#[inline]
pub fn linear_to_gamma(linear_component: f32) -> f32
{
    linear_component.sqrt()
}

pub fn near_zero(v: Vector3<f32>) -> bool
{
    let s = 1e-8;
    (v.x.abs() < s) && (v.y.abs() < s) && (v.z.abs() < s)
}

#[inline]
pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32>
{
    v - 2.0 * dot(v, n) * n
}

#[inline]
pub fn refract(uv: Vector3<f32>, n: Vector3<f32>, etai_over_etat: f32) -> Vector3<f32>
{
    let cos_thera = dot(-uv, n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_thera * n);
    let r_out_parallel = abs(1.0 - length_squared(r_out_perp)).sqrt() * -1.0 * n;
    r_out_perp + r_out_parallel
}

#[inline]
pub fn degrees_to_radians(degrees: f32) -> f32
{
    degrees * PI / 180.0
}

pub fn set_face_normal(r: Ray, outward_normal: Vector3<f32>) -> (Vector3<f32>, bool)
{
    // Sets the hit record normal vector
    // NOTE: the parameter `outward_normal` is assumed to have unit length
    let front_face = dot(r.direction(), outward_normal) < 0.0;
    if front_face
    {
        (outward_normal, front_face)
    }
    else
    {
        (-outward_normal, front_face)
    }
}