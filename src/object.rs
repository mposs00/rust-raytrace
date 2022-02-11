use crate::vector;

pub struct Material {
    pub diffuse_color: Vec<f32>
}

pub trait Object {
    fn ray_intersect(&self, origin: &Vec<f32>, direction: &Vec<f32>) -> (bool, f32);
    fn get_center(&self) -> Vec<f32>;
    fn get_material(&self) -> Material;
}

pub struct Sphere {
    pub center: Vec<f32>,
    pub radius: f32,
    pub material: Material
}

impl Object for Sphere {
    fn ray_intersect(&self, origin: &Vec<f32>, direction: &Vec<f32>) -> (bool, f32) {
        let l: Vec<f32> = vector::sub_vector(&self.center, origin);
        let tca: f32 = vector::dot_product(&l, direction);
        let d2: f32 = vector::dot_product(&l, &l) - (tca * tca);
        if d2 > (self.radius * self.radius) {
            return (false, 0.);
        }
        let thc: f32 = f32::sqrt((self.radius * self.radius) - d2);
        let mut t0: f32 = tca - thc;
        let t1: f32 = tca + thc;
        if t0 < 0. {
            t0 = t1;
        }

        if t0 < 0. {
            return (false, t0);
        }
        else {
            return (true, t0);
        }
    }

    fn get_center(&self) -> Vec<f32> {
        self.center.clone()
    }

    fn get_material(&self) -> Material {
        Material {
            diffuse_color: self.material.diffuse_color.clone()
        }
    }
}