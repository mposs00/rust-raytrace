use crate::vector;

pub struct Material {
    pub diffuse_color: Vec<f32>,
    pub specular_color: Vec<f32>,
    pub specular_exp: f32
}

pub trait Object : Send + Sync + Clone {
    fn ray_intersect(&self, origin: &Vec<f32>, direction: &Vec<f32>) -> RayIntersection;
    fn get_center(&self) -> Vec<f32>;
    fn get_material(&self) -> Material;
}

pub struct Sphere {
    pub center: Vec<f32>,
    pub radius: f32,
    pub material: Material
}

pub struct Plane {
    pub center: Vec<f32>,
    pub normal: Vec<f32>,
    pub material: Material
}

pub struct RayIntersection {
    pub distance: f32,
    pub normal: Vec<f32>,
    pub intersect_point: Vec<f32>,
    pub did_intersect: bool
}

impl Object for Plane {
    fn ray_intersect(&self, origin: &Vec<f32>, direction: &Vec<f32>) -> RayIntersection {
        let denom: f32 = vector::dot_product(&self.normal, direction);
        let mut t: f32 = 0.;

        if denom.abs() > 0.0000001 {
            let p0l0: Vec<f32> = vector::sub_vector(&self.center, origin);
            t = vector::dot_product(&p0l0, &self.normal) / denom;
        }

        RayIntersection {
            distance: t,
            normal: self.normal.clone(),
            did_intersect: t >= 0.,
            intersect_point: vector::add_vector(origin, &vector::scale(direction, t))
        }
    }

    fn get_center(&self) -> Vec<f32> {
        self.center.clone()
    }

    fn get_material(&self) -> Material {
        Material {
            diffuse_color: self.material.diffuse_color.clone(),
            specular_color: self.material.specular_color.clone(),
            specular_exp: self.material.specular_exp
        }
    }
}

impl Object for Sphere {
    fn ray_intersect(&self, origin: &Vec<f32>, direction: &Vec<f32>) -> RayIntersection {
        let l: Vec<f32> = vector::sub_vector(&self.center, origin);
        let tca: f32 = vector::dot_product(&l, direction);
        let d2: f32 = vector::dot_product(&l, &l) - (tca * tca);
        if d2 > (self.radius * self.radius) {
            return RayIntersection {
                did_intersect: false,
                intersect_point: vec![],
                normal: vec![],
                distance: 0.
            }
        }
        let thc: f32 = f32::sqrt((self.radius * self.radius) - d2);
        let mut t0: f32 = tca - thc;
        let t1: f32 = tca + thc;
        if t0 < 0. {
            t0 = t1;
        }
        let hit: Vec<f32> = vector::add_vector(origin, &vector::scale(&direction, t0));
        RayIntersection {
            did_intersect: !(t0 < 0.),
            intersect_point: hit.clone(),
            distance: t0,
            normal: vector::normalize(&vector::sub_vector(&hit.clone(), &self.get_center()))
        }
    }

    fn get_center(&self) -> Vec<f32> {
        self.center.clone()
    }

    fn get_material(&self) -> Material {
        Material {
            diffuse_color: self.material.diffuse_color.clone(),
            specular_color: self.material.specular_color.clone(),
            specular_exp: self.material.specular_exp
        }
    }
}