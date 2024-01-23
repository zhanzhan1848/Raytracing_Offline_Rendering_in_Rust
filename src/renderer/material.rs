use cgmath::{Array, dot, Vector3};
use cgmath::num_traits::pow;
use crate::renderer::custom_function::{near_zero, random_double, random_unit_vector3, reflect, refract, unit_vector3};
use crate::renderer::hittable::HitRecord;
use crate::renderer::ray::Ray;

pub trait Material: Sync + Send
{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool;
}


#[derive(Clone)]
pub struct Lambertian
{
    albedo: Vector3<f32>
}

impl Lambertian
{
    pub fn new(color: Vector3<f32>) -> Self
    {
        Lambertian
        {
            albedo: color
        }
    }

    pub fn get_albedo(&self) -> Vector3<f32>
    {
        self.albedo
    }


}

impl Material for Lambertian
{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool
    {
        let mut scatter_direction = rec.get_normal() + random_unit_vector3();
        if near_zero(scatter_direction)
        {
            scatter_direction = rec.get_normal();
        }
        let r = Ray::new(rec.get_point(), scatter_direction);
        scattered.clone_from(&r);
        let albedo = self.get_albedo();
        attenuation.clone_from(&albedo);
        true
    }
}

#[derive(Clone)]
pub struct Metal
{
    albedo: Vector3<f32>,
    fuzz: f32
}

impl Metal
{
    pub fn new(v: Vector3<f32>, f: f32) -> Self
    {
        Metal
        {
            albedo: v,
            fuzz: f.min(1.0)
        }
    }

    pub fn get_albedo(&self) -> Vector3<f32>
    {
        self.albedo
    }

    pub fn get_fuzz(&self) -> f32
    {
        self.fuzz
    }
}

impl Material for Metal
{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool
    {
        let reflected = reflect(unit_vector3(r_in.direction()), rec.get_normal());
        let r = Ray::new(rec.get_point(), reflected + self.fuzz * random_unit_vector3());
        scattered.clone_from(&r);
        let albedo = self.get_albedo();
        attenuation.clone_from(&albedo);
        dot(scattered.direction(), rec.get_normal()) > 0.0
    }
}

#[derive(Clone)]
pub struct Dielectric
{
    ior: f32
}

impl Dielectric
{
    pub fn new(ior: f32) -> Self
    {
        Dielectric
        {
            ior: ior
        }
    }

    pub fn reflectance(cosine: f32, ref_idx: f32) -> f32
    {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * pow((1.0 - cosine), 5)
    }
}

impl Material for Dielectric
{
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vector3<f32>, scattered: &mut Ray) -> bool {
        attenuation.clone_from(&Vector3::from_value(1.0));
        let mut refraction_ratio : f32;
        if rec._front_face
        {
            refraction_ratio = 1.0 / self.ior;
        }
        else
        {
            refraction_ratio = self.ior;
        }

        let unit_direction = unit_vector3(r_in.direction());
        let cos_theta = dot(-unit_direction, rec.get_normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let mut direction: Vector3<f32>;

        if cannot_refract || Dielectric::reflectance(cos_theta.clone(), refraction_ratio.clone()) > random_double()
        {
            direction = reflect(unit_direction, rec.get_normal());
        }
        else
        {
            direction = refract(unit_direction, rec.get_normal(), refraction_ratio);
        }

        scattered.clone_from(&Ray::new(rec.get_point(), direction));
        true
    }
}