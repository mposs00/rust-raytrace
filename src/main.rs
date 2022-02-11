mod scene;
mod object;
mod ppm;
mod vector;
mod light;
use scene::*;

fn main() {
    let mut scene = Scene::new(1024, 768, 60.);

    scene.add_light(light::Light {
        position: vec![0.0, 25.0, -20.0],
        intensity: 0.75,
    });

    scene.add_light(light::Light {
        position: vec![0.0, 0.0, 0.0],
        intensity: 1.,
    });

    for x in (-3..=3).step_by(2) {
        for y in (-3..=3).step_by(2) {
            let dist: f32 = (y as f32 * 4.) + x as f32 * -1.;
            scene.add_object(Box::new(object::Sphere {
                center: vec![x as f32 * 1.5, y as f32 * 1.25, -20. + dist],
                material: object::Material {
                    diffuse_color: vec![0.4, 0.4, 0.3],
                    specular_color: vec![1., 1., 1.],
                    specular_exp: 100.
                },
                radius: 1.
            }));     
        }
    }

    scene.add_object(Box::new(object::Plane {
        center: vec![0., -10., -15.],
        normal: vec![0., 1., 0.],
        material: object::Material {
            diffuse_color: vec![0.2, 0.2, 0.5],
            specular_color: vec![1., 1., 1.],
            specular_exp: 20.
        }
    }));

    /*scene.add_object(Box::new(object::Sphere {
        center: vec![0., 10., -20.],
        material: object::Material {
            diffuse_color: vec![0.4, 0.4, 0.3]
        },
        radius: 5.
    }));

    scene.add_object(Box::new(object::Sphere {
        center: vec![-1., -1.5, -12.],
        material: object::Material {
            diffuse_color: vec![0.4, 0.4, 0.3]
        },
        radius: 2.
    }));

    scene.add_object(Box::new(object::Sphere {
        center: vec![1.5, -0.5, -18.],
        material: object::Material {
            diffuse_color: vec![0.3, 0.1, 0.1]
        },
        radius: 2.
    }));

    scene.add_object(Box::new(object::Sphere {
        center: vec![7., 5., -18.],
        material: object::Material {
            diffuse_color: vec![0.3, 0.1, 0.1]
        },
        radius: 2.
    }));

    scene.add_object(Box::new(object::Plane {
        center: vec![0., 0., 0.],
        normal: vec![0., 0., 0.],
        material: object::Material {
            diffuse_color: vec![0.2, 0.2, 0.5]
        }
    }));*/
    scene.render();
    scene.save();
}