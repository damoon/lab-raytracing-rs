use crate::{
    materials::Material,
    tuples::{color, dot, reflect, Tuple},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Pointlight {
    pub position: Tuple,
    pub intensity: Tuple,
}

impl Pointlight {
    pub fn new(position: Tuple, intensity: Tuple) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

pub fn lighting(
    material: &Material,
    light: &Pointlight,
    point: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
    in_shadow: bool,
) -> Tuple {
    let black = color(0.0, 0.0, 0.0);
    let distance = (light.position - point).magnitude();
    let intensity = light.intensity * 10000.0 / (distance * distance);
    let effective_color = material.color * intensity;
    let lightv = (light.position - point).normalize();
    let ambient = effective_color * material.ambient;

    if in_shadow {
        return ambient;
    }

    let light_dot_normal = dot(&lightv, normalv);
    let mut diffuse = black;
    let mut specular = black;
    if light_dot_normal > 0.0 {
        diffuse = effective_color * material.diffuse * light_dot_normal;
        let reflectv = reflect(&-lightv, normalv);
        let reflect_dot_eye = dot(&reflectv, eyev);

        if reflect_dot_eye > 0.0 {
            let factor = f64::powf(reflect_dot_eye, material.shininess);
            specular = intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}
