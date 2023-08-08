use crate::shape::{HitRecord, Hittable};
use crate::vectors::Vec3;
use crate::ray::Ray;
use crate::material::Material;
use crate::shape::triangle::Triangle;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Mesh<M: Material> {
    pub triangles: Vec<Triangle<M>>,
}

impl<M: Material + Clone> Mesh<M> {
    pub fn new(triangles: Vec<Triangle<M>>) -> Self {
        Self { triangles }
    }

    pub fn load(filepath: String, material: M) -> Self {
        let file = File::open(filepath).unwrap();
        let reader = BufReader::new(file);

        let mut vertices: Vec<Vec3<f64>> = Vec::new();
        let mut triangles: Vec<Triangle<M>> = Vec::new();

        for res in reader.lines() {
            let line = res.unwrap();
            if let [cmd, ref args @ ..] = line.split_whitespace().collect::<Vec<_>>()[..] {
                match cmd {
                    "v" => {
                        vertices.push(Vec3::new(
                                args[0].parse().unwrap(),
                                args[1].parse().unwrap(),
                                args[2].parse().unwrap()
                        ));
                    },
                    "vt" => {
                    },
                    "vn" => {
                    },
                    "f" => {
                        triangles.push(Triangle::new([
                                vertices[args[0].parse::<usize>().unwrap()-1],
                                vertices[args[1].parse::<usize>().unwrap()-1],
                                vertices[args[2].parse::<usize>().unwrap()-1]
                        ], material.clone()));
                    },
                    "g" => {
                    },
                    "usemtl" => {
                    },
                    _ => {}
                }
            }
        }

        Self { triangles }
    }
}

impl<M: Material + Send + Sync> Hittable for Mesh<M> {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        let mut result = None;
        let mut closest = t_max;

        for triangle in &self.triangles {
            if let Some(h) = triangle.hit(ray, (t_min, closest)) {
                result = Some(h);
                closest = h.t;
            }
        }

        result
    }
}
