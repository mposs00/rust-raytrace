mod frame;
mod vec3;
mod render;
mod object;
mod scene;
mod light;
mod threadpool;
use vec3::Vec3f;
use frame::Frame;
use frame::FrameBuffer;
use render::Render;
use scene::Scene;
use object::*;
use light::Light;

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;
const FOV: f32 = 80.;

fn main() {
    let frame = Frame::new(WIDTH, HEIGHT, FOV);
    let mut framebuf = frame.init_frame();
    let mut scene = Scene {
        frame: frame,
        objects: vec![],
        lights: vec![],
        bg_color: Vec3f {
            x: 0.6,
            y: 0.6,
            z: 0.8
        }
    };

    scene.objects.push(Box::new(Plane {
        center: Vec3f::new(0., -10., 0.),
        normal: Vec3f::new(0., 1., 0.),
        material: Material {
            diffuse_color: Vec3f::new(1., 0., 0.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 10.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Plane {
        center: Vec3f::new(0., 10., 0.),
        normal: Vec3f::new(0., -1., 0.),
        material: Material {
            diffuse_color: Vec3f::new(0., 1., 0.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 10.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Plane {
        center: Vec3f::new(0., 0., -20.),
        normal: Vec3f::new(0., 0., 1.),
        material: Material {
            diffuse_color: Vec3f::new(1., 1., 0.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 10.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Plane {
        center: Vec3f::new(10., 0., 0.),
        normal: Vec3f::new(-1., 0., 0.),
        material: Material {
            diffuse_color: Vec3f::new(1., 0., 1.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 10.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Plane {
        center: Vec3f::new(-10., 0., 0.),
        normal: Vec3f::new(1., 0., 0.),
        material: Material {
            diffuse_color: Vec3f::new(0., 1., 1.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 10.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Sphere {
        center: Vec3f::new(-2., -8.5, -12.),
        radius: 2.5,
        material: Material {
            diffuse_color: Vec3f::new(0., 0., 1.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 50.,
            reflective: false
        }
    }));

    scene.objects.push(Box::new(Sphere {
        center: Vec3f::new(5., -5., -15.),
        radius: 4.,
        material: Material {
            diffuse_color: Vec3f::new(1., 1., 1.),
            specular_color: Vec3f::new(1., 1., 1.),
            specular_exp: 50.,
            reflective: true
        }
    }));

    /*for i in (-3..=3).step_by(2) {
        for j in (-3..=3).step_by(2) {
            scene.objects.push(Box::new(Sphere {
                center: Vec3f::new(i as f32 * 1.5, j as f32 * 1.5, -10.),
                radius: 1.,
                material: Material {
                    diffuse_color: Vec3f::new(0., 0., 1.),
                    specular_color: Vec3f::new(1., 1., 1.),
                    specular_exp: 50.
                }
            }));
        }
    }*/

    scene.lights.push(Light {
        position: Vec3f::new(-9., 9., -19.),
        intensity: 0.5
    });

    scene.lights.push(Light {
        position: Vec3f::zero(),
        intensity: 0.5
    });

    Render::render(&scene, &mut framebuf);
    frame.save(framebuf, "out.ppm");
}
