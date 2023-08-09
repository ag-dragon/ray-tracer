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
        let mut texture_cords: Vec<(f64, f64)> = Vec::new();
        let mut normals: Vec<Vec3<f64>> = Vec::new();
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
                        texture_cords.push((
                                args[0].parse().unwrap(),
                                args[1].parse().unwrap()
                        ));
                    },
                    "vn" => {
                        normals.push(Vec3::new(
                                args[0].parse().unwrap(),
                                args[1].parse().unwrap(),
                                args[2].parse().unwrap()
                        ));
                    },
                    "f" => {
                        let verts: Vec<Vec<usize>> = args.iter().map( |indices| {
                            let split: Vec<&str> = indices.split('/').collect();
                            split.iter().map( |index| {
                                match index.parse::<usize>().ok() {
                                    Some(parsed) => parsed - 1,
                                    None => !0
                                }
                            }).collect()
                        }).collect();

                        if verts[0].len() == 1 {
                            triangles.push(Triangle::new([
                                    vertices[verts[0][0]],
                                    vertices[verts[1][0]],
                                    vertices[verts[2][0]],
                            ], None, None, material.clone()));
                        } else {
                            triangles.push(Triangle::new([
                                    vertices[verts[0][0]],
                                    vertices[verts[1][0]],
                                    vertices[verts[2][0]]
                                    ], Some([
                                    normals[verts[0][2]],
                                    normals[verts[1][2]],
                                    normals[verts[2][2]]
                                    ]), Some([
                                    texture_cords[verts[0][1]],
                                    texture_cords[verts[1][1]],
                                    texture_cords[verts[2][1]]
                            ]), material.clone()));
                        }
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
