use crate::{
    canvas::Canvas,
    intersections::color_at,
    matrices::{identity_matrix, Matrix4x4},
    rays::Ray,
    tuples::point,
    world::World,
};
pub const RAY_RECURSION_DEPTH: usize = 5;

pub struct Camera {
    pub hsize: usize,
    pub vsize: usize,
    pub field_of_view: f64,
    transform: Matrix4x4,
    transform_inverse: Matrix4x4,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
    pub ray_recursion_depth: usize,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let transform = identity_matrix();
        let transform_inverse = transform.inverse().unwrap();

        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;
        let mut half_width = half_view * aspect;
        let mut half_height = half_view;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        }
        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            field_of_view,
            transform,
            transform_inverse,
            pixel_size,
            half_width,
            half_height,
            ray_recursion_depth: RAY_RECURSION_DEPTH,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4x4) {
        self.transform = transform;
        self.transform_inverse = self.transform.inverse().unwrap();
    }

    pub fn transform(&self) -> &Matrix4x4 {
        &self.transform
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size;
        let yoffset = (py as f64 + 0.5) * self.pixel_size;
        // the untransformed coordinates of the pixel in world space.
        // (remember that the camera looks toward -z, so +x is to the *left*.)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;
        // using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1)
        let pixel = &self.transform_inverse * point(world_x, world_y, -1.0);
        let origin = &self.transform_inverse * point(0.0, 0.0, 0.0);
        let direction = (pixel - &origin).normalize();
        Ray::new(origin, direction)
    }
}

pub fn render(camera: &Camera, world: &World) -> Canvas {
    let mut image = Canvas::new(camera.hsize, camera.vsize);
    for y in 0..camera.vsize {
        for x in 0..camera.hsize {
            let ray = camera.ray_for_pixel(x, y);
            let color = color_at(world, &ray, camera.ray_recursion_depth, None);
            image.set(x, y, color);
        }
    }
    image
}
