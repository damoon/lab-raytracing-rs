use lab_raytracing_rs::camera::Camera;
use lab_raytracing_rs::groups::Group;
use lab_raytracing_rs::lights::Pointlight;
use lab_raytracing_rs::materials::Material;
use lab_raytracing_rs::objects::default_plane;
use lab_raytracing_rs::objects::default_sphere;
use lab_raytracing_rs::objects::glass;
use lab_raytracing_rs::objects::metallic;
use lab_raytracing_rs::objects::mirror;
use lab_raytracing_rs::patterns::checkers_pattern;
use lab_raytracing_rs::patterns::solid_pattern;
use lab_raytracing_rs::transformations::rotation_y;
use lab_raytracing_rs::transformations::scaling;
use lab_raytracing_rs::transformations::translation;
use lab_raytracing_rs::transformations::view_transform;
use lab_raytracing_rs::tuples::color;
use lab_raytracing_rs::tuples::point;
use lab_raytracing_rs::tuples::vector;
use lab_raytracing_rs::world::World;
use std::f64::consts::PI;
use std::io;

fn main() -> io::Result<()> {
    let white = Box::new(solid_pattern(color(1.0, 1.0, 1.0)));
    let yellow = Box::new(solid_pattern(color(1.0, 1.0, 0.0)));
    let white_color = color(1.0, 1.0, 1.0);

    let mut world = World::default();

    let mut checkers = checkers_pattern(white, yellow);
    checkers.set_transform(rotation_y(PI / 3.0) * scaling(0.5, 0.5, 0.5));
    let mut floor = default_plane();
    // floor.material.pattern = Some(ring_pattern(red, grey));
    floor.material.pattern = Some(Box::new(checkers));
    floor.material.specular = 0.1;
    world.add_object(floor);

    // 4 16 64 256
    let mut rand = fastrand::Rng::with_seed(0);

    // -16..16
    // 256 spheres
    for lvl_3_offset_x in [-8.0, 8.0] {
        for lvl_3_offset_z in [-8.0, 8.0] {
            let mut group_lvl_3 = Group::default();
            group_lvl_3.set_transform(translation(lvl_3_offset_x, 0.0, lvl_3_offset_z));

            // -8..8
            // 64 spheres
            for lvl_2_offset_x in [-4.0, 4.0] {
                for lvl_2_offset_z in [-4.0, 4.0] {
                    let mut group_lvl_2 = Group::default();
                    group_lvl_2.set_transform(translation(lvl_2_offset_x, 0.0, lvl_2_offset_z));

                    // -4..4
                    // 16 spheres
                    for lvl_1_offset_x in [-2.0, 2.0] {
                        for lvl_1_offset_z in [-2.0, 2.0] {
                            let mut group_lvl_1 = Group::default();
                            group_lvl_1.set_transform(translation(
                                lvl_1_offset_x,
                                0.0,
                                lvl_1_offset_z,
                            ));

                            // -2..2
                            // 4 spheres
                            for offset_x in [-1.0, 1.0] {
                                for offset_z in [-1.0, 1.0] {
                                    let mut marble = default_sphere();
                                    marble.material = match rand.f64() {
                                        x if x > 0.0 && x < 0.25 => glass(),
                                        x if x > 0.25 && x < 0.5 => mirror(),
                                        x if x > 0.5 && x < 0.75 => {
                                            let mut m = metallic();
                                            m.color = color(rand.f64(), rand.f64(), rand.f64());
                                            m
                                        }
                                        // 0.75..1.0
                                        _ => {
                                            let mut m = Material::default();
                                            m.color = color(rand.f64(), rand.f64(), rand.f64());
                                            m
                                        }
                                    };
                                    let radius = (rand.f64() * 0.5) + 0.2; // 0.2 - 0.7
                                    marble.set_transform(
                                        translation(offset_x, radius, offset_z)
                                            * scaling(radius, radius, radius),
                                    );
                                    group_lvl_1.add_object(marble)
                                }
                            }

                            group_lvl_2.add_group(group_lvl_1);
                        }
                    }
                    group_lvl_3.add_group(group_lvl_2);
                }
            }
            world.add_group(group_lvl_3);
        }
    }

    let mut camera = Camera::new(1600, 900, PI / 3.0);
    camera.set_transform(view_transform(
        &point(-22.0, 22.0, -22.0),
        &point(-6.66, 0.0, -6.66),
        &vector(0.0, 1.0, 0.0),
    ));
    world.light = Some(Pointlight::new(point(20.0, 20.0, -20.0), white_color));

    let canvas = camera.render(&world);

    let file = &mut io::stdout();
    let writer = &mut io::BufWriter::with_capacity(1024 * 128, file);
    canvas.ppm(writer)?;

    Ok(())
}
