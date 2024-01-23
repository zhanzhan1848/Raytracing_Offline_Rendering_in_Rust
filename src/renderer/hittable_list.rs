use std::rc::Rc;
use crate::renderer::hittable::{HitRecord, Hittable};
use crate::renderer::interval::Interval;
use crate::renderer::ray::Ray;

#[derive(Clone)]
pub struct HittableList
{
    pub objects: Vec<Rc<dyn Hittable>>
}

impl Hittable for HittableList
{
    // fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool
    // {
    //     let mut temp_rec: HitRecord;
    //     let mut hit_anything = false;
    //     let mut closet_so_far = ray_t.max();
    //
    //     for object in &self.objects
    //     {
    //         if object.hit(ray, Interval::new(ray_t.min(), closet_so_far), &mut temp_rec)
    //         {
    //             hit_anything = true;
    //             closet_so_far = temp_rec.clone().get_t();
    //             *rec = temp_rec.clone();
    //         }
    //     }
    //
    //     hit_anything
    // }

    fn hit(&self, ray: Ray, ray_t: Interval) -> Option<HitRecord>
    {
        let mut hit_anything: Option<HitRecord> = None;
        let mut closet_so_far = ray_t.max();

        for object in &self.objects
        {
            if let Some(hit) = object.hit(ray, Interval::new(ray_t.min(), closet_so_far))
            {
                closet_so_far = hit._t;
                hit_anything = Some(hit);
            }
        }

        hit_anything
    }
}

impl HittableList
{
    pub fn new() -> Self
    {
        HittableList
        {
            objects: vec![]
        }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>)
    {
        self.objects.push(object)
    }

    pub fn clear(&mut self)
    {
        self.objects.clear()
    }
}

impl Drop for HittableList
{
    fn drop(&mut self) {
        self.clear()
    }
}