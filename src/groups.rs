use crate::{
    intersections::Intersection,
    materials::Material,
    matrices::{identity_matrix, Matrix4x4},
    objects::Object,
    rays::Ray,
    tuples::{point, Tuple},
};
use auto_ops::impl_op_ex;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub enum GroupMember {
    SubGroup(Arc<Group>),
    Object(Arc<Object>),
}

impl GroupMember {
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        match self {
            GroupMember::SubGroup(g) => g.intersect(ray),
            GroupMember::Object(o) => o.intersect(ray, o),
        }
    }

    pub fn bounds(&self) -> &Option<AABB> {
        match self {
            GroupMember::SubGroup(g) => g.bounds(),
            GroupMember::Object(o) => o.bounds(),
        }
    }

    fn update_transform(&self, update: &Matrix4x4) -> Self {
        match self {
            GroupMember::SubGroup(g) => {
                let mut g = g.as_ref().clone();
                g.set_transform(update * g.transform());
                GroupMember::SubGroup(Arc::new(g))
            }
            GroupMember::Object(o) => {
                let mut o = o.as_ref().clone();
                o.set_transform(update * o.transform());
                GroupMember::Object(Arc::new(o))
            }
        }
    }

    fn set_material(&self, m: &Material) -> Self {
        match self {
            GroupMember::SubGroup(g) => {
                let mut g = g.as_ref().clone();
                g.set_material(m);
                GroupMember::SubGroup(Arc::new(g))
            }
            GroupMember::Object(o) => {
                let mut o = o.as_ref().clone();
                o.material = m.clone();
                GroupMember::Object(Arc::new(o))
            }
        }
    }

    fn objects(&self) -> Vec<Arc<Object>> {
        match self {
            GroupMember::SubGroup(g) => g.objects(),
            GroupMember::Object(o) => {
                vec![o.clone()]
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    transform: Matrix4x4,
    transform_inverse: Matrix4x4,
    bounds: Option<AABB>,
    elements: Vec<GroupMember>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AABB {
    pub min: Tuple,
    pub max: Tuple,
}

impl AABB {
    fn goes_to_infinity(&self) -> bool {
        if self.min.x == f64::NEG_INFINITY {
            return true;
        }
        if self.min.y == f64::NEG_INFINITY {
            return true;
        }
        if self.min.z == f64::NEG_INFINITY {
            return true;
        }
        if self.max.x == f64::INFINITY {
            return true;
        }
        if self.max.y == f64::INFINITY {
            return true;
        }
        if self.max.z == f64::INFINITY {
            return true;
        }
        false
    }
}

impl_op_ex!(+|a: &AABB, b: &AABB| -> AABB {
    let mut min = a.min.clone();
    let mut max = a.max.clone();

    // min
    if b.min.x < min.x {
        min.x = b.min.x;
    }
    if b.min.y < min.y {
        min.y = b.min.y;
    }
    if b.min.z < min.z {
        min.z = b.min.z;
    }

    // max
    if b.max.x > max.x {
        max.x = b.max.x;
    }
    if b.max.y > max.y {
        max.y = b.max.y;
    }
    if b.max.z > max.z {
        max.z = b.max.z;
    }

    AABB { min, max }
});

impl Group {
    pub fn default() -> Self {
        let transform = identity_matrix();
        let transform_inverse = transform.inverse().unwrap();
        let bounds = None;
        let elements = Vec::new();
        Group {
            transform,
            transform_inverse,
            bounds,
            elements,
        }
    }

    pub fn regroup_aabb(self) -> Self {
        let mut group = Group::default();

        let mut objects = Vec::new();
        // let mut bounds = None;
        let mut min = point(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = point(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for e in self.objects() {
            if e.bounds().clone().unwrap().goes_to_infinity() {
                group.add_object(e.as_ref().clone());
            } else {
                let eb = e.bounds().clone().unwrap();
                let center = eb.center();

                if center.x < min.x {
                    min.x = center.x;
                }
                if center.y < min.y {
                    min.y = center.y;
                }
                if center.z < min.z {
                    min.z = center.z;
                }
                if center.x > max.x {
                    max.x = center.x;
                }
                if center.y > max.y {
                    max.y = center.y;
                }
                if center.z > max.z {
                    max.z = center.z;
                }

                objects.push(e)
            }
        }

        let x_range = max.x - min.x;
        let y_range = max.y - min.y;
        let z_range = max.z - min.z;
        let bigish = Self::bigish(x_range, y_range, z_range);
        let mut group_low = Group::default();
        let mut group_high = Group::default();

        if bigish == 3 {
            return self;
        } else if bigish == 0 {
            let average = (max.x + min.x) / 2.0;
            for e in objects {
                let eb = e.bounds().clone().unwrap();
                let e = e.as_ref().clone();
                // let x = (eb.max.x + eb.min.x) / 2.0;
                let x = eb.center().x;
                if x < average {
                    group_low.add_object(e)
                } else {
                    group_high.add_object(e)
                }
            }
        } else if bigish == 1 {
            let average = (max.y + min.y) / 2.0;
            for e in objects {
                let eb = e.bounds().clone().unwrap();
                let e = e.as_ref().clone();
                let y = eb.center().y;
                if y < average {
                    group_low.add_object(e)
                } else {
                    group_high.add_object(e)
                }
            }
        } else {
            let average = (max.z + min.z) / 2.0;
            for e in objects {
                let eb = e.bounds().clone().unwrap();
                let e = e.as_ref().clone();
                let z = eb.center().z;
                if z < average {
                    group_low.add_object(e)
                } else {
                    group_high.add_object(e)
                }
            }
        }

        if group_low.len() >= 4 {
            group_low = group_low.regroup_aabb();
        }
        if group_high.len() >= 4 {
            group_high = group_high.regroup_aabb();
        }

        group.add_group(group_low);
        group.add_group(group_high);

        group
    }

    fn bigish(x: f64, y: f64, z: f64) -> usize {
        if x > y && x > z {
            return 0;
        }
        if y > x && y > z {
            return 1;
        }
        if z > x && z > y {
            return 2;
        }
        if x > y {
            return 0;
        }
        if y > x {
            return 1;
        }
        if z > x {
            return 2;
        }
        3
    }

    fn objects(&self) -> Vec<Arc<Object>> {
        let mut ls = Vec::new();
        for e in self.elements.iter() {
            ls.append(&mut e.objects());
        }
        ls
    }

    pub fn add_group(&mut self, mut e: Group) {
        e.set_transform(&self.transform * e.transform());
        self.bounds = Self::outer_bounds(&self.bounds, e.bounds());
        let e = GroupMember::SubGroup(Arc::new(e));
        self.elements.push(e);
    }

    pub fn add_object(&mut self, mut e: Object) {
        e.set_transform(&self.transform * e.transform());
        self.bounds = Self::outer_bounds(&self.bounds, e.bounds());
        let e = GroupMember::Object(Arc::new(e));
        self.elements.push(e)
    }

    pub fn get_object(&self, idx: usize) -> Arc<Object> {
        match &self.elements.get(idx).unwrap() {
            GroupMember::Object(o) => o.clone(),
            GroupMember::SubGroup(_) => panic!("found a group"),
        }
    }

    pub fn set_material(&mut self, m: &Material) {
        self.elements = self.elements.iter().map(|e| e.set_material(m)).collect()
    }

    fn outer_bounds(this: &Option<AABB>, other: &Option<AABB>) -> Option<AABB> {
        match (this, other) {
            (None, None) => None,
            (None, Some(b)) => Some(b.clone()),
            (Some(a), None) => Some(a.clone()),
            (Some(a), Some(b)) => Some(a + b),
        }
    }

    // pub fn contains_object(&self, o: Arc<Object>) -> bool {
    //     self.elements.contains(&GroupMember::Object(o))
    // }

    pub fn contains_group(&self, g: &Group) -> bool {
        for e in self.elements.iter() {
            if let GroupMember::SubGroup(sg) = e {
                let sg = sg.as_ref();
                if g == sg {
                    return true;
                }
            }
        }

        false
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn transform(&self) -> &Matrix4x4 {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix4x4) {
        let update = &transform * &self.transform_inverse;
        self.transform = transform;
        self.transform_inverse = self.transform.inverse().unwrap();

        self.elements = self
            .elements
            .iter()
            .map(|e| e.update_transform(&update))
            .collect();

        let mut bounds = None;
        for e in self.elements.iter() {
            bounds = Self::outer_bounds(&bounds, e.bounds());
        }
        self.bounds = bounds;
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        match &self.bounds {
            None => {
                return Vec::new();
            }
            Some(b) => {
                if !b.is_intersected(ray) {
                    return Vec::new();
                }
            }
        }

        let mut xs = Vec::new();
        for element in self.elements.iter() {
            let ls = &mut element.intersect(ray);
            xs.append(ls);
        }
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    pub fn bounds(&self) -> &Option<AABB> {
        &self.bounds
    }
}

impl AABB {
    pub fn is_intersected(&self, ray: &Ray) -> bool {
        let (xtmin, xtmax) =
            Self::check_axis(self.min.x, self.max.x, ray.origin.x, ray.direction.x);
        let (ytmin, ytmax) =
            Self::check_axis(self.min.y, self.max.y, ray.origin.y, ray.direction.y);

        let tmin = if xtmin > ytmin { xtmin } else { ytmin };
        let tmax = if xtmax < ytmax { xtmax } else { ytmax };

        if tmin > tmax {
            return false;
        }

        let (ztmin, ztmax) =
            Self::check_axis(self.min.z, self.max.z, ray.origin.z, ray.direction.z);

        let tmin = if ztmin > tmin { ztmin } else { tmin };
        let tmax = if ztmax < tmax { ztmax } else { tmax };

        if tmin > tmax {
            return false;
        }

        true
    }

    pub fn center(&self) -> Tuple {
        let avg_x = (self.min.x + self.max.x) / 2.0;
        let avg_y = (self.min.y + self.max.y) / 2.0;
        let avg_z = (self.min.z + self.max.z) / 2.0;
        point(avg_x, avg_y, avg_z)
    }

    fn check_axis(min: f64, max: f64, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;
        let (mut tmin, mut tmax) = if direction.abs() >= f64::EPSILON {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            (
                tmin_numerator * f64::INFINITY,
                tmax_numerator * f64::INFINITY,
            )
        };
        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax)
        }
        (tmin, tmax)
    }

    pub fn mul_aabb(&self, m: &Matrix4x4) -> AABB {
        let points = [
            point(self.min.x, self.min.y, self.min.z) * m,
            point(self.min.x, self.min.y, self.max.z) * m,
            point(self.min.x, self.max.y, self.min.z) * m,
            point(self.min.x, self.max.y, self.max.z) * m,
            point(self.max.x, self.min.y, self.min.z) * m,
            point(self.max.x, self.min.y, self.max.z) * m,
            point(self.max.x, self.max.y, self.min.z) * m,
            point(self.max.x, self.max.y, self.max.z) * m,
        ];

        let mut min = point(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = point(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for p in points {
            // min
            if p.x < min.x {
                min.x = p.x;
            }
            if p.y < min.y {
                min.y = p.y;
            }
            if p.z < min.z {
                min.z = p.z;
            }

            // max
            if p.x > max.x {
                max.x = p.x;
            }
            if p.y > max.y {
                max.y = p.y;
            }
            if p.z > max.z {
                max.z = p.z;
            }
        }

        AABB { min, max }
    }
}
