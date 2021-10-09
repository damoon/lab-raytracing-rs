use crate::{
    patterns::Pattern,
    tuples::{color, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Tuple,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub pattern: Option<Pattern>,
}

impl Material {
    pub fn default() -> Self {
        Self {
            color: color(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            pattern: None,
        }
    }
}
