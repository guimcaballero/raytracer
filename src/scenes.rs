use crate::{camera::*, hittable::*, hittable_list::*, material::*, perlin::*, texture, vec3::*};
use rand::Rng;

#[allow(dead_code)]
pub enum Scene {
    ManySpheres,
    TwoPerlinSpheres,
    Earth,
    LightRectangle,
    CornellBox,
}

pub struct World {
    pub hittables: HittableList,
    pub camera: Camera,
    pub background_color: Color,
}

pub fn generate_world(scene: Scene, aspect_ratio: f64) -> World {
    match scene {
        Scene::ManySpheres => many_spheres(aspect_ratio),
        Scene::TwoPerlinSpheres => two_perlin_spheres(aspect_ratio),
        Scene::Earth => earth(aspect_ratio),
        Scene::LightRectangle => light_rectangle(aspect_ratio),
        Scene::CornellBox => cornell_box(aspect_ratio),
    }
}

fn many_spheres(aspect_ratio: f64) -> World {
    let mut hittables = HittableList::new();

    // Ground
    let checker = texture::checker(
        texture::solid_color(Color::new(0.2, 0.3, 0.1)),
        texture::solid_color(Color::new(0.9, 0.9, 0.9)),
    );
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::LambertianTexture(checker),
    }));

    // Spheres
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Dielectric(1.5),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::LambertianTexture(texture::noise(Perlin::new(), 4.0)),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(4.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Color::new(0.8, 0.6, 0.2), 0.0),
    }));

    // Illuminated sphere
    hittables.add(Box::new(Sphere {
        center: Point::new(-4.0, 0.5, 2.0),
        radius: 0.5,
        material: Material::Lambertian(Color::new(2.0, 2.0, 1.0)),
    }));

    for a in -5..5 {
        for b in -5..5 {
            let choose_mat = rand::thread_rng().gen::<f64>();
            let center = Point {
                x: a as f64 + 0.9 * rand::thread_rng().gen::<f64>(),
                y: 0.2,
                z: b as f64 + 0.9 * rand::thread_rng().gen::<f64>(),
            };

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.7 {
                    let albedo = Color::random() * Color::random();
                    let center1 =
                        center + Vec3::new(0.0, rand::thread_rng().gen_range(0.0, 0.3), 0.0);
                    hittables.add(Box::new(MovingSphere {
                        center0: center,
                        center1,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        material: Material::Lambertian(albedo),
                    }));
                } else if choose_mat < 0.8 {
                    let texture = texture::image("earthmap.jpg");
                    hittables.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::LambertianTexture(texture),
                    }));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = rand::thread_rng().gen_range(0.0, 0.5);
                    hittables.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Metal(albedo, fuzz),
                    }));
                } else {
                    hittables.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        material: Material::Dielectric(1.5),
                    }));
                }
            }
        }
    }

    World {
        hittables,
        camera: default_camera(aspect_ratio),
        background_color: Color::new(0.7, 0.8, 1.0),
    }
}

fn two_perlin_spheres(aspect_ratio: f64) -> World {
    let mut hittables = HittableList::new();

    let texture = texture::marble(Perlin::new(), 4.0);
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 1.0, 0.0),
        radius: 1.0,
        material: Material::Metal(Color::new(1.0, 1.0, 1.0), 0.0),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::LambertianTexture(texture.clone()),
    }));

    World {
        hittables,
        camera: default_camera(aspect_ratio),
        background_color: Color::new(0.7, 0.8, 1.0),
    }
}

fn earth(aspect_ratio: f64) -> World {
    let mut hittables = HittableList::new();

    let texture = texture::image("earthmap.jpg");
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 0.0, 0.0),
        radius: 2.0,
        material: Material::LambertianTexture(texture),
    }));

    World {
        hittables,
        camera: default_camera(aspect_ratio),
        background_color: Color::new(0.7, 0.8, 1.0),
    }
}

fn light_rectangle(aspect_ratio: f64) -> World {
    let mut hittables = HittableList::new();

    // Two marble spheres
    let texture = texture::marble(Perlin::new(), 4.0);
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 2.0, 0.0),
        radius: 2.0,
        material: Material::Metal(Color::new(1.0, 1.0, 1.0), 0.0),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        material: Material::LambertianTexture(texture.clone()),
    }));
    hittables.add(Box::new(Sphere {
        center: Point::new(0.0, 5.0, 0.0),
        radius: 1.0,
        material: Material::DiffuseLight(Color::new(4.0, 3.0, 1.0)),
    }));

    hittables.add(Box::new(XYRect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
        material: Material::DiffuseLight(Color::new(4.0, 4.0, 4.0)),
    }));

    let lookfrom = Point::new(26.0, 3.0, 6.0);
    let lookat = Point::new(0.0, 2.0, 0.0);
    let dist_to_focus = 10.0;
    let vfov = 20.0;
    let aperture = 0.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        background_color: Color::zeros(),
    }
}

fn cornell_box(aspect_ratio: f64) -> World {
    let mut hittables = HittableList::new();

    // Walls
    hittables.add(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::new(0.12, 0.45, 0.15)),
    }));
    hittables.add(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: Material::Lambertian(Color::new(0.65, 0.05, 0.05)),
    }));
    hittables.add(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        material: Material::Lambertian(Color::new(0.73, 0.73, 0.73)),
    }));
    hittables.add(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::new(0.73, 0.73, 0.73)),
    }));
    hittables.add(Box::new(XYRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        material: Material::Lambertian(Color::new(0.73, 0.73, 0.73)),
    }));

    // Light
    hittables.add(Box::new(XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 550.0,
        material: Material::DiffuseLight(Color::new(15.0, 15.0, 15.0)),
    }));

    let lookfrom = Point::new(278.0, 278.0, -800.0);
    let lookat = Point::new(278.0, 278.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let vfov = 40.0;
    let aperture = 0.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    World {
        hittables,
        camera,
        background_color: Color::zeros(),
    }
}

fn default_camera(aspect_ratio: f64) -> Camera {
    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let dist_to_focus = 10.0;
    let vfov = 20.0;
    let aperture = 0.0;

    Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    )
}
