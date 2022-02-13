use crate::frame::Frame;
use crate::frame::FrameBuffer;
use crate::vec3::Vec3f;
use crate::scene::Scene;
use crate::object::RayIntersection;
use crate::object::Object;
use crate::object::Material;
use crate::threadpool::ThreadPool;

pub struct Render;

impl Render {
    fn check_intersections(scene: &Scene, origin: Vec3f, direction: Vec3f) -> (RayIntersection, Material) {
        let mut closest_intersection = RayIntersection {
            did_intersect: false,
            distance: f32::MAX,
            normal: Vec3f::zero(),
            intersect_point: Vec3f::zero()
        };
        let mut closest_material: Material = Material {
            diffuse_color: Vec3f::zero(),
            specular_color: Vec3f::zero(),
            specular_exp: 0.,
            reflective: false
        };
        
        for obj in &scene.objects {
            let intersection = obj.ray_intersect(origin, direction);
            if intersection.did_intersect && intersection.distance < closest_intersection.distance {
                closest_intersection = intersection;
                closest_material = obj.get_material();
            }
        }

        closest_intersection.did_intersect = closest_intersection.distance < 1000.;
        (closest_intersection, closest_material)
    }

    fn calc_lights(scene: &Scene, intersection: &RayIntersection, material: &Material, direction: Vec3f) -> Vec3f {
        let mut diffuse_intensity: f32 = 0.;
        let mut specular_intensity: f32 = 0.;

        for light in &scene.lights {
            let light_direction = (light.position - intersection.intersect_point).normalize();
            let light_dist = (light.position - intersection.intersect_point).norm();

            // Specular lighting
            let phong_factor = f32::max(0., Vec3f::reflect(light_direction, intersection.normal) * direction).powf(material.specular_exp);

            // Shadows
            let mut shadow_origin = Vec3f::zero();
            if light_direction * intersection.normal < 0. {
                shadow_origin = intersection.intersect_point - (intersection.normal * 0.0001);
            }
            else {
                shadow_origin = intersection.intersect_point + (intersection.normal * 0.0001);
            }

            let shadow_intersection = Render::check_intersections(scene, shadow_origin, light_direction).0;
            if shadow_intersection.did_intersect && (shadow_intersection.intersect_point - shadow_origin).norm() < light_dist {
                continue;
            }

            diffuse_intensity += light.intensity * f32::max(0., light_direction * intersection.normal);
            specular_intensity += phong_factor * light.intensity;
        }

        let diffuse_component = material.diffuse_color * diffuse_intensity;
        let specular_component = material.specular_color * specular_intensity;
        diffuse_component + specular_component
    }

    pub fn calc_pixel(scene: &Scene, origin: Vec3f, direction: Vec3f, depth: i32) -> Vec3f {
        let (intersection, material) = Render::check_intersections(scene, origin, direction); 

        if depth < 4 && intersection.did_intersect {
            let mut lighting_color = Render::calc_lights(scene, &intersection, &material, direction);
            
            if material.reflective {
                let reflect_direction = Vec3f::reflect(direction, intersection.normal);
                let mut reflect_origin = Vec3f::zero();
                if reflect_direction * intersection.normal < 0. {
                    reflect_origin = intersection.intersect_point - intersection.normal * 0.0001;
                }
                else {
                    reflect_origin = intersection.intersect_point + intersection.normal * 0.0001;
                }
                lighting_color = (lighting_color * 0.3) + (Render::calc_pixel(scene, reflect_origin, reflect_direction, depth + 1) * 0.7);
            }

            return lighting_color;
        }

        scene.bg_color
    }

    pub fn render(scene: &Scene, framebuf: &mut FrameBuffer) {
        println!("x:{} y:{}", scene.frame.width, scene.frame.height);
        //let pool = ThreadPool::new(8);

        for j in 0..scene.frame.height {
            for i in 0..scene.frame.width {
                let x: f32 = (2.*(i as f32 + 0.5)/scene.frame.width as f32  - 1.)*f32::tan(scene.frame.fov_deg.to_radians()/2.)*scene.frame.width as f32/scene.frame.height as f32;
                let y: f32 = -(2.*(j as f32 + 0.5)/scene.frame.height as f32 - 1.)*f32::tan(scene.frame.fov_deg.to_radians()/2.);

                let camera_direction = Vec3f::new(x, y, -1.).normalize();

                let casted_color = Render::calc_pixel(scene, Vec3f::zero(), camera_direction, 0);
                Frame::set_pixel(framebuf, i, j, casted_color);
            }
        }
    }
}