use crate::{
    canvas::Canvas,
    intersections::color_at,
    matrices::{identity_matrix, Matrix4x4},
    rays::Ray,
    tuples::{color, point, Tuple},
    world::World,
};
use crossbeam_channel::bounded;
pub const RAY_RECURSION_DEPTH: usize = 5;
use std::env;

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
    pub antialiasing: AntiAliasing,
    pub renderer: Renderer,
}

pub enum AntiAliasing {
    Off,
    Fast,
    Raster(usize),
    Dynamic,
}

pub enum Renderer {
    SingleThreaded,
    Multithreaded(usize),
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

        let antialiasing = AntiAliasing::from_env();
        let renderer = Renderer::from_env();

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
            antialiasing,
            renderer,
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4x4) {
        self.transform = transform;
        self.transform_inverse = self.transform.inverse().unwrap();
    }

    pub fn transform(&self) -> &Matrix4x4 {
        &self.transform
    }

    pub fn color_at_pixel(&self, world: &World, x: usize, y: usize) -> Tuple {
        self.antialiasing.color_at_pixel(self, world, x, y)
    }

    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
        self.ray_for_pixel_offseted(px, py, 0.0, 0.0)
    }

    fn ray_for_pixel_offseted(&self, px: usize, py: usize, offsetx: f64, offsety: f64) -> Ray {
        // the offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5 + offsetx) * self.pixel_size;
        let yoffset = (py as f64 + 0.5 + offsety) * self.pixel_size;
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

    pub fn render(&self, world: &World) -> Canvas {
        self.renderer.render(self, world)
    }
}

impl AntiAliasing {
    pub fn from_env() -> Self {
        match env::var("ANTIALIASING") {
            Err(_) => Self::Fast,
            Ok(antialiasing) => match antialiasing.to_lowercase().as_str() {
                "" => Self::Fast,
                "off" => Self::Off,
                "fast" => Self::Fast,
                "dynamic" => Self::Dynamic,
                s => Self::Raster(s.parse::<usize>().unwrap()),
            },
        }
    }

    pub fn color_at_pixel(&self, camera: &Camera, world: &World, x: usize, y: usize) -> Tuple {
        match self {
            AntiAliasing::Off => Self::off(camera, world, x, y),
            AntiAliasing::Fast => Self::fast(camera, world, x, y),
            AntiAliasing::Raster(n) => Self::rastered(camera, world, x, y, *n),
            AntiAliasing::Dynamic => Self::dynamic(camera, world, x, y),
        }
    }

    fn off(camera: &Camera, world: &World, x: usize, y: usize) -> Tuple {
        let ray = camera.ray_for_pixel(x, y);
        color_at(world, &ray, camera.ray_recursion_depth, None)
    }

    fn fast(camera: &Camera, world: &World, x: usize, y: usize) -> Tuple {
        let ray1 = camera.ray_for_pixel_offseted(x, y, -0.33, 0.33);
        let color1 = color_at(world, &ray1, camera.ray_recursion_depth, None);
        let ray3 = camera.ray_for_pixel_offseted(x, y, 0.33, 0.33);
        let color3 = color_at(world, &ray3, camera.ray_recursion_depth, None);
        let ray7 = camera.ray_for_pixel_offseted(x, y, -0.33, -0.33);
        let color7 = color_at(world, &ray7, camera.ray_recursion_depth, None);
        let ray9 = camera.ray_for_pixel_offseted(x, y, 0.33, -0.33);
        let color9 = color_at(world, &ray9, camera.ray_recursion_depth, None);
        (color1 + color3 + color7 + color9) / 4.0
    }

    fn dynamic(camera: &Camera, world: &World, x: usize, y: usize) -> Tuple {
        let (mut summed_color, mut ray_count) = Self::color_at_pixel_borders(camera, world, x, y);

        while ray_count < 50 {
            let summed_color_previous = summed_color.clone();
            let ray_count_previous = ray_count;

            let color = Self::color_at_pixel_random(camera, world, x, y, ray_count);
            summed_color = &summed_color + &color;
            ray_count += ray_count;

            let magnitude = ((summed_color_previous / ray_count_previous as f64)
                - (&summed_color / ray_count as f64))
                .magnitude();
            if magnitude < 0.01 {
                return summed_color / ray_count as f64;
            }
        }

        summed_color / ray_count as f64
    }

    fn color_at_pixel_borders(
        camera: &Camera,
        world: &World,
        x: usize,
        y: usize,
    ) -> (Tuple, usize) {
        let points = [(0.1, 0.1), (0.11, 0.9), (0.9, 0.11), (0.91, 0.91)];
        let mut summed_color = color(0.0, 0.0, 0.0);
        for (offsetx, offsety) in points {
            let ray = camera.ray_for_pixel_offseted(x, y, -0.5 + offsetx, -0.5 + offsety);
            let color = color_at(world, &ray, camera.ray_recursion_depth, None);
            summed_color = summed_color + color;
        }

        (summed_color, points.len())
    }

    // returns the sum of all random rays combined
    fn color_at_pixel_random(
        camera: &Camera,
        world: &World,
        x: usize,
        y: usize,
        n: usize,
    ) -> Tuple {
        let rand = fastrand::Rng::with_seed((x.pow(2) + y.pow(3) + n.pow(4)) as u64);
        let mut color = color(0.0, 0.0, 0.0);
        for _ in 0..n {
            let ray = camera.ray_for_pixel_offseted(x, y, -0.5 + rand.f64(), -0.5 + rand.f64());
            let c = color_at(world, &ray, camera.ray_recursion_depth, None);
            color = color + c;
        }

        color
    }

    pub fn rastered(camera: &Camera, world: &World, x: usize, y: usize, n: usize) -> Tuple {
        let offset = 1.0 / n as f64;
        let corner = -0.5 + (offset / 2.0);
        let mut color = color(0.0, 0.0, 0.0);
        for x2 in 0..n {
            for y2 in 0..n {
                let ray = camera.ray_for_pixel_offseted(
                    x,
                    y,
                    corner + (x2 as f64 * offset),
                    corner + (y2 as f64 * offset),
                );
                let c = color_at(world, &ray, camera.ray_recursion_depth, None);
                color = color + c;
            }
        }

        color / n.pow(2) as f64
    }
}

impl Renderer {
    pub fn from_env() -> Self {
        match env::var("CORES") {
            Err(_) => Self::Multithreaded(num_cpus::get()),
            Ok(antialiasing) => match antialiasing.to_lowercase().as_str() {
                "" => Self::Multithreaded(num_cpus::get()),
                "off" => Self::SingleThreaded,
                cores => Self::Multithreaded(cores.parse::<usize>().unwrap()),
            },
        }
    }

    pub fn render(&self, camera: &Camera, world: &World) -> Canvas {
        match self {
            Self::SingleThreaded => Self::render_singlethreaded(camera, world),
            Self::Multithreaded(cores) => Self::render_multithreaded(camera, world, *cores),
        }
    }

    fn render_singlethreaded(camera: &Camera, world: &World) -> Canvas {
        let mut image = Canvas::new(camera.hsize, camera.vsize);
        for row in 0..camera.vsize {
            dbg!(row);
            for x in 0..camera.hsize {
                let color = camera.color_at_pixel(world, x, row);
                image.set(x, row, color);
            }
        }
        image
    }

    fn render_multithreaded(camera: &Camera, world: &World, cores: usize) -> Canvas {
        let (sender, receiver) = bounded::<(usize, Canvas)>(10);
        let (sender_row, receiver_row) = bounded::<usize>(camera.vsize);

        let image = crossbeam::scope(|scope| {
            scope.spawn(move |_| {
                for y in 0..camera.vsize {
                    sender_row.send(y).unwrap();
                }
            });

            for _ in 0..cores {
                let thread_receiver = receiver_row.clone();
                let thread_sender = sender.clone();
                let thread_camera = &camera;
                let thread_world = &world;
                scope.spawn(move |_| {
                    for row_id in thread_receiver {
                        //let canvas = Canvas::new(camera.hsize, 1);
                        let canvas = Self::render_row(thread_camera, thread_world, row_id);
                        thread_sender.send((row_id, canvas)).unwrap();
                    }
                });
            }
            drop(sender);

            let image = scope
                .spawn(move |_| {
                    let mut image = Canvas::new(camera.hsize, camera.vsize);
                    for (row_id, row) in receiver {
                        for x in 0..camera.hsize {
                            let c = row.at(x, 0);
                            image.set(x, row_id, color(c.x, c.y, c.z));
                        }
                    }
                    image
                })
                .join()
                .unwrap();

            image
        })
        .unwrap();

        image
    }

    fn render_row(camera: &Camera, world: &World, row: usize) -> Canvas {
        let y = row;
        let mut image = Canvas::new(camera.hsize, 1);
        for x in 0..camera.hsize {
            let color = camera.color_at_pixel(world, x, y);
            image.set(x, 0, color);
        }
        dbg!(row);
        image
    }
}
