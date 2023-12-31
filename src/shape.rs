pub mod sphere;
pub mod triangle;
pub mod mesh;
pub mod plane;
pub use self::sphere::Sphere;
pub use self::triangle::Triangle;
pub use self::mesh::Mesh;
pub use self::plane::Plane;

use crate::material::Material;
use crate::vector::Vec3;
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct HitRecord<'material> {
    pub point: Vec3<f64>,
    pub normal: Vec3<f64>,
    pub material: &'material dyn Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_range: (f64, f64)) -> Option<HitRecord>;
}
