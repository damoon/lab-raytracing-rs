use crate::{
    materials::Material,
    patterns::pattern_at_shape,
    shapes::Object,
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
    object: &Object,
    light: &Pointlight,
    point: &Tuple,
    eyev: &Tuple,
    normalv: &Tuple,
    in_shadow: bool,
) -> Tuple {
    let material_color = match &material.pattern {
        None => material.color.clone(),
        Some(pattern) => pattern_at_shape(pattern, object, point),
    };

    let black = color(0.0, 0.0, 0.0);
    let effective_color = &material_color * &light.intensity;
    let lightv = (&light.position - point).normalize();
    let ambient = &effective_color * material.ambient;

    if in_shadow {
        return ambient;
    }

    let light_dot_normal = dot(&lightv, normalv);
    let mut diffuse = black.clone();
    let mut specular = black;
    if light_dot_normal > 0.0 {
        diffuse = &effective_color * material.diffuse * light_dot_normal;
        let reflectv = reflect(&-lightv, normalv);
        let reflect_dot_eye = dot(&reflectv, eyev);

        if reflect_dot_eye > 0.0 {
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = &light.intensity * material.specular * factor;
        }
    }

    ambient + diffuse + specular
}
