use crate::{
    matrices::{identity_matrix, Matrix4x4},
    objects::Object,
    tuples::{point, Tuple},
};
use noise::{NoiseFn, Perlin, Seedable};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    transform: Matrix4x4,
    transform_inverse: Matrix4x4,
    pub renderer: Renderer,
}

#[derive(Debug, Clone)]
pub enum Renderer {
    Stripes(Box<Pattern>, Box<Pattern>),
    Gradient(Box<Pattern>, Box<Pattern>),
    Ring(Box<Pattern>, Box<Pattern>),
    Checkers(Box<Pattern>, Box<Pattern>),
    RadialGradient(Box<Pattern>, Box<Pattern>),
    Blended(Box<Pattern>, Box<Pattern>),
    Perturbed(f64, Box<Perlin>, Box<Perlin>, Box<Perlin>, Box<Pattern>),
    Solid(Tuple),
    Test(),
}

impl PartialEq for Renderer {
    fn eq(&self, other: &Renderer) -> bool {
        match (self, other) {
            (Renderer::Stripes(a1, b1), Renderer::Stripes(a2, b2)) => a1 == a2 && b1 == b2,
            (Renderer::Gradient(a1, b1), Renderer::Gradient(a2, b2)) => a1 == a2 && b1 == b2,
            (Renderer::Ring(a1, b1), Renderer::Ring(a2, b2)) => a1 == a2 && b1 == b2,
            (Renderer::Checkers(a1, b1), Renderer::Checkers(a2, b2)) => a1 == a2 && b1 == b2,
            (Renderer::RadialGradient(a1, b1), Renderer::RadialGradient(a2, b2)) => {
                a1 == a2 && b1 == b2
            }
            (Renderer::Blended(a1, b1), Renderer::Blended(a2, b2)) => a1 == a2 && b1 == b2,
            (
                Renderer::Perturbed(scale1, x1, y1, z1, pattern1),
                Renderer::Perturbed(scale2, x2, y2, z2, pattern2),
            ) => {
                scale1 == scale2
                    && x1.seed() == x2.seed()
                    && y1.seed() == y2.seed()
                    && z1.seed() == z2.seed()
                    && pattern1 == pattern2
            }
            (Renderer::Solid(a), Renderer::Solid(b)) => a == b,
            (Renderer::Test(), Renderer::Test()) => true,
            (_, _) => false,
        }
    }
}

impl Pattern {
    pub fn new(transform: Matrix4x4, renderer: Renderer) -> Pattern {
        let transform_inverse = transform.inverse().unwrap();
        Pattern {
            transform,
            transform_inverse,
            renderer,
        }
    }

    pub fn color_at(&self, p: &Tuple) -> Tuple {
        self.renderer.color_at(p)
    }

    pub fn set_transform(&mut self, transform: Matrix4x4) {
        self.transform = transform;
        self.transform_inverse = self.transform.inverse().unwrap();
    }

    pub fn transform(&self) -> &Matrix4x4 {
        &self.transform
    }
}

impl Renderer {
    fn color_at(&self, p: &Tuple) -> Tuple {
        match self {
            Renderer::Stripes(a, b) => {
                if p.x.floor() % 2.0 == 0.0 {
                    return a.color_at(&(p * &a.transform_inverse));
                }
                b.color_at(&(p * &b.transform_inverse))
            }
            Renderer::Gradient(a, b) => {
                let a = &a.color_at(&(p * &a.transform_inverse));
                let b = &b.color_at(&(p * &b.transform_inverse));
                let distance = b - a;
                let fraction = p.x - p.x.floor();
                a + distance * fraction
            }
            Renderer::Ring(a, b) => {
                if (p.x * p.x + p.z * p.z).sqrt().floor() % 2.0 == 0.0 {
                    return a.color_at(&(p * &a.transform_inverse));
                }
                b.color_at(&(p * &b.transform_inverse))
            }
            Renderer::Checkers(a, b) => {
                let px = (p.x + (2.0 * f64::EPSILON)).floor();
                let py = (p.y + (2.0 * f64::EPSILON)).floor();
                let pz = (p.z + (2.0 * f64::EPSILON)).floor();
                if ((px + py + pz) % 2.0).abs() < f64::EPSILON {
                    return a.color_at(&(p * &a.transform_inverse));
                }
                b.color_at(&(p * &b.transform_inverse))
            }
            Renderer::RadialGradient(a, b) => {
                let a = &a.color_at(&(p * &a.transform_inverse));
                let b = &b.color_at(&(p * &b.transform_inverse));
                let distance = b - a;
                let fraction = (p - point(0.0, 0.0, 0.0)).magnitude() % 1.0;
                a + distance * fraction
            }
            Renderer::Blended(a, b) => {
                (a.color_at(&(p * &a.transform_inverse)) + b.color_at(&(p * &b.transform_inverse)))
                    / 2.0
            }
            Renderer::Perturbed(scale, x, y, z, pattern) => {
                let point_3d = [p.x, p.y, p.z];
                let x = p.x + x.get(point_3d) * scale;
                let y = p.y + y.get(point_3d) * scale;
                let z = p.z + z.get(point_3d) * scale;
                pattern.color_at(&(point(x, y, z) * &pattern.transform_inverse))
            }
            Renderer::Solid(a) => a.clone(),
            Renderer::Test() => p.clone(),
        }
    }
}

pub fn solid_pattern(color: Tuple) -> Pattern {
    Pattern {
        transform: identity_matrix(),
        transform_inverse: identity_matrix().inverse().unwrap(),
        renderer: Renderer::Solid(color),
    }
}

pub fn stripe_pattern(a: Box<Pattern>, b: Box<Pattern>) -> Pattern {
    Pattern {
        transform: identity_matrix(),
        transform_inverse: identity_matrix().inverse().unwrap(),
        renderer: Renderer::Stripes(a, b),
    }
}

pub fn gradient_pattern(a: Box<Pattern>, b: Box<Pattern>) -> Pattern {
    Pattern {
        transform: identity_matrix(),
        transform_inverse: identity_matrix().inverse().unwrap(),
        renderer: Renderer::Gradient(a, b),
    }
}

pub fn ring_pattern(a: Box<Pattern>, b: Box<Pattern>) -> Pattern {
    Pattern {
        transform: identity_matrix(),
        transform_inverse: identity_matrix().inverse().unwrap(),
        renderer: Renderer::Ring(a, b),
    }
}

pub fn checkers_pattern(a: Box<Pattern>, b: Box<Pattern>) -> Pattern {
    Pattern {
        transform: identity_matrix(),
        transform_inverse: identity_matrix().inverse().unwrap(),
        renderer: Renderer::Checkers(a, b),
    }
}

pub fn radial_gradient_pattern(a: Box<Pattern>, b: Box<Pattern>) -> Pattern {
    Pattern {
        transform: identity_matrix(),
        transform_inverse: identity_matrix().inverse().unwrap(),
        renderer: Renderer::RadialGradient(a, b),
    }
}

pub fn test_pattern() -> Pattern {
    Pattern {
        transform: identity_matrix(),
        transform_inverse: identity_matrix().inverse().unwrap(),
        renderer: Renderer::Test(),
    }
}

pub fn pattern_at_shape(pattern: &Pattern, object: &Arc<Object>, world_point: &Tuple) -> Tuple {
    let object_point = object.transform_inverse() * world_point;
    let pattern_point = &pattern.transform_inverse * object_point;
    pattern.color_at(&pattern_point)
}
