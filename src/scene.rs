use crate::object::*;
use crate::ppm::Frame;
use crate::vector;
use crate::light::Light;
use crate::threads::ThreadPool;
use std::collections::HashMap;
use std::sync::mpsc;

#[derive(Clone)]
pub struct Camera {
    pub width: usize,
    pub height: usize,
    pub fov_deg: f32,
    pub pos: Vec<f32>,
    pub frame: Frame
}

#[derive(Clone)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
    pub bg_color: Vec<f32>
}

pub struct Renderer;

impl Camera {
    pub fn new(width: usize, height: usize, fov_deg: f32) -> Camera {
        Camera {
            width: width,
            height: height,
            fov_deg: fov_deg,
            pos: vec![0., 0., 0.],
            frame: Frame::new(width, height)
        }
    }
}

impl Scene {
    pub fn new(width: usize, height: usize, fov_deg: f32) -> Scene {
        Scene {
            camera: Camera::new(width, height, fov_deg),
            objects: vec![],
            lights: vec![],
            bg_color: vec![0.2, 0.2, 0.2]
        }
    }

    pub fn add_object(&mut self, obj_box: Box<dyn Object>) {
        self.objects.push(obj_box);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn save(self) {
        self.camera.frame.save("./out.ppm");
    }
}

impl Renderer {
    fn scene_intersect(objects: Vec<Box<dyn Object>>, origin: &Vec<f32>, direction: &Vec<f32>) -> (bool, Vec<f32>, Vec<f32>, Material) {
        let mut dist: f32 = f32::MAX;
        let mut mtrl = Material {
            diffuse_color: vec![],
            specular_color: vec![],
            specular_exp: 0.
        };
        let mut hit: Vec<f32> = vec![];
        let mut n: Vec<f32> = vec![];
        for obj in objects {
            let mut dist_i: f32 = dist;
            let intersection = obj.ray_intersect(origin, direction);
            dist_i = intersection.distance;
            if intersection.did_intersect && dist_i < dist {
                dist = dist_i;
                hit = intersection.intersect_point;
                n = intersection.normal;
                mtrl = obj.get_material();
            }
        }
        //println!("distance to obj: {}", dist);
        (dist < 1000., hit, n, mtrl)
    }

    fn cast_ray(scene: &Scene, origin: &Vec<f32>, direction: &Vec<f32>) -> Vec<f32> {
        let intersection = Renderer::scene_intersect(scene.objects, origin, direction);
        if intersection.0 {
            let mut diffuse_intensity: f32 = 0.;
            let mut specular_intensity: f32 = 0.;
            for light in &scene.lights {
                let light_dir = vector::normalize(&vector::sub_vector(&light.position, &intersection.1));
                let light_dist: f32 = vector::norm(&vector::sub_vector(&light.position, &intersection.1));

                let mut shadow_origin: Vec<f32> = vec![];
                if vector::dot_product(&light_dir, &intersection.2) < 0. {
                    shadow_origin = vector::sub_vector(&intersection.1, &vector::scale(&intersection.2, 0.0001));
                }
                else {
                    shadow_origin = vector::add_vector(&intersection.1, &vector::scale(&intersection.2, 0.0001));
                }
                let shadow_intersection = Renderer::scene_intersect(scene.objects, &shadow_origin, &light_dir);
                if shadow_intersection.0 && vector::norm(&vector::sub_vector(&shadow_intersection.1, &shadow_origin)) < light_dist {
                    continue;
                }

                diffuse_intensity += light.intensity * f32::max(0., vector::dot_product(&light_dir, &intersection.2));
                specular_intensity += f32::max(0., vector::dot_product(&vector::reflect(&light_dir, &intersection.2), direction)).powf(intersection.3.specular_exp) * light.intensity;
                //println!("dot product of light dir and surface normal: {}", vector::dot_product(&light_dir, &intersection.2));
                //println!("diffuse intensity: {}", diffuse_intensity);
            }
            let diffuse_component = vector::scale(&intersection.3.diffuse_color, diffuse_intensity);
            let specular_component = vector::scale(&intersection.3.specular_color, specular_intensity);
            return vector::add_vector(&diffuse_component, &specular_component)
        }
        scene.bg_color
    }

    pub fn render(scene: &'static mut Scene) {
        let pool = ThreadPool::new(8);

        for j in 0..scene.camera.height {
            for i in 0..scene.camera.width {
                let x: f32 =  (2.*(i as f32 + 0.5)/scene.camera.width as f32  - 1.)*f32::tan(scene.camera.fov_deg.to_radians()/2.)*scene.camera.width as f32/scene.camera.height as f32;
                let y: f32 = -(2.*(j as f32 + 0.5)/scene.camera.height as f32 - 1.)*f32::tan(scene.camera.fov_deg.to_radians()/2.);

                pool.execute(move || {
                    let direction: Vec<f32> = vector::normalize(&vec![x, y, -1.]);
                    let casted_color = Renderer::cast_ray(scene, &scene.camera.pos, &direction);
                    scene.camera.frame.set_pixel_rgb(i, j, &casted_color);
                });

                //self.camera.frame.set_pixel_rgb(i, j, &casted_color);
            }
        }
    }

}