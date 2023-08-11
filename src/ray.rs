use crate::vector::{Vec3, Color};
use crate::scene::Scene;
use crate::shape::Hittable;

use crate::texture::Texture;
use crate::shape::Sphere;
use crate::material::Metal;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3<f64>,
    pub direction: Vec3<f64>,
}

impl Ray {
    pub fn new(origin: Vec3<f64>, direction: Vec3<f64>) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3<f64> {
        self.origin + (self.direction*t)
    }

    pub fn color(&self, scene: &Scene, depth: i32) -> Color {
        // Reached max depth
        if depth <= 0 {
            return Color::zero();
        }

        // Check if ray hits any other objects in scene
        if let Some(hit_record) = scene.hit(self, (0.001, f64::INFINITY)) {
            let emitted = hit_record.material.emitted(hit_record.u, hit_record.v);
            if let Some(scatter) = hit_record.material.scatter(self, &hit_record) {
                return emitted + scatter.attenuation * scatter.scattered.color(scene, depth-1);
            } else {
                return emitted;
            }
        }

        // Background color
        let unit_direction = self.direction.normalized();
        match &scene.skybox {
            Some(sky) => {
                let (u, v) = Sphere::<Metal>::get_uv(unit_direction);
                let c = sky.color(u, v);
                Color::new(
                    c.x * c.x,
                    c.y * c.y,
                    c.z * c.z
                )
            },
            None => {
                let t = (unit_direction.y + 1.0) * 0.5;
                Color::one()*(1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let r = Ray::new(
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(4.0, 5.0, 6.0)
        );
        assert_eq!(r.origin.x, 1.0);
        assert_eq!(r.origin.y, 2.0);
        assert_eq!(r.origin.z, 3.0);
        assert_eq!(r.direction.x, 4.0);
        assert_eq!(r.direction.y, 5.0);
        assert_eq!(r.direction.z, 6.0);
    }

    #[test]
    fn test_at() {
        let r = Ray::new(
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 2.0, 3.0)
        );
        let p = r.at(0.5);
        assert_eq!(p.x, 0.5);
        assert_eq!(p.y, 1.0);
        assert_eq!(p.z, 1.5);
    }
}
