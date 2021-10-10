use crate::{
    patterns::Pattern,
    tuples::{color, Tuple},
};

pub const REFRACTIVE_INDEX_VACUUM: f64 = 1.0;
pub const REFRACTIVE_INDEX_AIR: f64 = 1.00029;
pub const REFRACTIVE_INDEX_WATER: f64 = 1.333;
pub const REFRACTIVE_INDEX_GLASS: f64 = 1.52;
pub const REFRACTIVE_INDEX_DIAMOND: f64 = 2.417;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Tuple,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
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
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }
}
