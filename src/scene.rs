use crate::object::*;
use crate::ppm::Frame;
use crate::vector;
use crate::light::Light;

pub struct Camera {
    pub width: usize,
    pub height: usize,
    pub fov_deg: f32,
    pub pos: Vec<f32>,
    pub frame: Frame
}

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Box<dyn Object>>,
    pub lights: Vec<Light>,
    pub bg_color: Vec<f32>
}

impl Scene {
    pub fn new(width: usize, height: usize, fov_deg: f32) -> Scene {
        Scene {
            camera: Camera {
                width: width,
                height: height,
                fov_deg: fov_deg,
                pos: vec![0., 0., 0.],
                frame: Frame::new(width, height)
            },
            objects: vec![],
            lights: vec![],
            bg_color: vec![0.2, 0.7, 0.8]
        }
    }
    
    fn scene_intersect(&mut self, origin: &Vec<f32>, direction: &Vec<f32>) -> (bool, Vec<f32>, Vec<f32>, Material) {
        let mut dist: f32 = f32::MAX;
        let mut mtrl = Material {
            diffuse_color: vec![]
        };
        let mut hit: Vec<f32> = vec![];
        let mut n: Vec<f32> = vec![];
        for obj in &self.objects {
            let mut dist_i: f32 = dist;
            let intersection = obj.ray_intersect(origin, direction);
            dist_i = intersection.1;
            if intersection.0 && dist_i < dist {
                dist = dist_i;
                hit = vector::add_vector(origin, &vector::scale(&direction, dist_i));
                n = vector::normalize(&vector::sub_vector(&hit, &obj.get_center()));
                mtrl = obj.get_material();
            }
        }
        (dist < 1000., hit, n, mtrl)
    }

    fn cast_ray(&mut self, origin: &Vec<f32>, direction: &Vec<f32>) -> Vec<f32> {
        let intersection = self.scene_intersect(origin, direction);
        if intersection.0 {
            let mut diffuse_intensity: f32 = 0.;
            for light in &self.lights {
                let light_dir = vector::normalize(&vector::sub_vector(&light.position, &intersection.1));
                diffuse_intensity += light.intensity * f32::max(0., vector::dot_product(&light_dir, &intersection.2));
            }
            return vector::scale(&intersection.3.diffuse_color, diffuse_intensity);
        }
        self.bg_color.clone()
    }

    pub fn add_object(&mut self, obj_box: Box<dyn Object>) {
        self.objects.push(obj_box);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn render(&mut self) {
        for j in 0..self.camera.height {
            for i in 0..self.camera.width {
                let x: f32 =  (2.*(i as f32 + 0.5)/self.camera.width as f32  - 1.)*f32::tan(self.camera.fov_deg.to_radians()/2.)*self.camera.width as f32/self.camera.height as f32;
                let y: f32 = -(2.*(j as f32 + 0.5)/self.camera.height as f32 - 1.)*f32::tan(self.camera.fov_deg.to_radians()/2.);
                let direction: Vec<f32> = vector::normalize(&vec![x, y, -1.]);
                let cam_pos = self.camera.pos.clone();
                let casted_color = &self.cast_ray(&cam_pos, &direction);
                self.camera.frame.set_pixel_rgb(i, j, casted_color);
            }
        }
    }

    pub fn save(self) {
        self.camera.frame.save("./out.ppm");
    }
}