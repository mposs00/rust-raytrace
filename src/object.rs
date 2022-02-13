use crate::vec3::Vec3f;

const EPSILON: f32 = 0.0000001;

#[derive(Copy, Clone)]
pub struct Material {
    pub diffuse_color: Vec3f,
    pub specular_color: Vec3f,
    pub specular_exp: f32,
    pub reflective: bool
}

pub trait Object : Send + Sync {
    fn ray_intersect(&self, origin: Vec3f, direction: Vec3f) -> RayIntersection;
    fn get_center(&self) -> Vec3f;
    fn get_material(&self) -> Material;
}

pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub material: Material
}

pub struct Plane {
    pub center: Vec3f,
    pub normal: Vec3f,
    pub material: Material
}

pub struct RayIntersection {
    pub distance: f32,
    pub normal: Vec3f,
    pub intersect_point: Vec3f,
    pub did_intersect: bool
}

impl Object for Plane {
    fn ray_intersect(&self, origin: Vec3f, direction: Vec3f) -> RayIntersection {
        let denom: f32 = self.normal * direction;
        let mut t: f32 = 0.;
        
        if denom.abs() > EPSILON {
            t = ((self.center - origin) * self.normal) / denom;
        }

        //println!("calc dist: {}", t);

        RayIntersection {
            distance: t,
            normal: self.normal,
            did_intersect: t >= 0.,
            intersect_point: origin + (direction * t)
        }
    }

    fn get_center(&self) -> Vec3f {
        self.center
    }

    fn get_material(&self) -> Material {
        self.material.clone()
    }
}

impl Object for Sphere {
    fn ray_intersect(&self, origin: Vec3f, direction: Vec3f) -> RayIntersection {
        let l = self.center - origin;
        let tca = l * direction;
        let d2 = (l * l) - (tca * tca);

        // Inside sphere
        if d2 > (self.radius * self.radius) {
            return RayIntersection {
                did_intersect: false,
                intersect_point: Vec3f::zero(),
                normal: Vec3f::zero(),
                distance: 0.
            }
        }

        let thc = f32::sqrt(self.radius * self.radius - d2);
        let mut t0 = tca - thc;
        let t1 = tca + thc;
        if t0 < 0. {
            t0 = t1;
        }

        let hit = origin + (direction * t0);
        
        RayIntersection {
            did_intersect: !(t0 < 0.),
            intersect_point: hit,
            distance: t0,
            normal: (hit - self.center).normalize()
        }
    }

    fn get_center(&self) -> Vec3f {
        self.center
    }

    fn get_material(&self) -> Material {
        self.material
    }
}