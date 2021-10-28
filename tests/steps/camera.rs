use crate::MyWorld;
use approx::assert_abs_diff_eq;
use cucumber::{given, then, when};
use lab_raytracing_rs::{
    camera::Camera,
    transformations::{rotation_y, translation, view_transform},
};
use std::f64::consts::PI;

#[given(regex = r"^(hsize|vsize) ← ([-0-9.]+)$")]
async fn set_usize(world: &mut MyWorld, target: String, value: usize) {
    world.usizes.insert(target, value);
}

#[given("field_of_view ← π/2")]
async fn set_float(world: &mut MyWorld) {
    world.floats.insert("field_of_view".to_string(), PI / 2.0);
}

#[when("c ← camera(hsize, vsize, field_of_view)")]
async fn create_camera(world: &mut MyWorld) {
    let hsize = *world.usizes.get("hsize").unwrap();
    let vsize = *world.usizes.get("vsize").unwrap();
    let field_of_view = *world.floats.get("field_of_view").unwrap();
    world.camera = Camera::new(hsize, vsize, field_of_view);
}

#[then(
    regex = r"^c.(hsize|vsize|pixel_size|field_of_view|transform) = ([-0-9.]+|π/2|identity_matrix)$"
)]
async fn compare_camera(world: &mut MyWorld, attribute: String, value: String) {
    match attribute.as_str() {
        "hsize" => assert_eq!(world.camera.hsize, value.parse::<usize>().unwrap()),
        "vsize" => assert_eq!(world.camera.vsize, value.parse::<usize>().unwrap()),
        "pixel_size" => assert_abs_diff_eq!(world.camera.pixel_size, value.parse::<f64>().unwrap()),
        "field_of_view" => {
            assert_abs_diff_eq!(world.camera.field_of_view, PI / 2.0)
        }
        "transform" => assert_eq!(world.camera.transform(), world.get4x4(&value)),
        _ => panic!("camera property not covered"),
    }
}

#[given(regex = r"^c ← camera\(([-0-9.]+), ([-0-9.]+), π/2\)$")]
async fn set_camera(world: &mut MyWorld, hsize: usize, vsize: usize) {
    let field_of_view = PI / 2.0;
    world.camera = Camera::new(hsize, vsize, field_of_view);
}

#[when(regex = r"^r ← ray_for_pixel\(c, ([-0-9.]+), ([-0-9.]+)\)$")]
async fn ray_for_pixel(world: &mut MyWorld, px: usize, py: usize) {
    world.r = world.camera.ray_for_pixel(px, py);
}

#[when("c.transform ← rotation_y(π/4) * translation(0, -2, 5)")]
async fn rotate_camera(world: &mut MyWorld) {
    let transform = rotation_y(PI / 4.0) * translation(0.0, -2.0, 5.0);
    world.camera.set_transform(transform);
}

#[given("c.transform ← view_transform(from, to, up)")]
async fn set_camera_transformation(world: &mut MyWorld) {
    let from = world.tuples.get("from").unwrap();
    let to = world.tuples.get("to").unwrap();
    let up = world.tuples.get("up").unwrap();
    let transform = view_transform(from, to, up);
    world.camera.set_transform(transform);
}

#[when("image ← render(c, w)")]
async fn render_image(world: &mut MyWorld) {
    world.image = world.camera.render(&world.w);
}
