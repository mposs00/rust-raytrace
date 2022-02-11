mod scene;
mod object;
mod ppm;
mod vector;
mod light;
use scene::*;

fn main() {
    let mut scene = Scene::new(1024, 768, 60.);

    scene.add_object(Box::new(object::Sphere {
        center: vec![-3., 0., -16.],
        material: object::Material {
            diffuse_color: vec![0.4, 0.4, 0.3]
        },
        radius: 2.
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

    scene.add_light(light::Light {
        position: vec![-20.0, 20.0, 20.0],
        intensity:1.5
    });

    scene.render();
    scene.save();
}