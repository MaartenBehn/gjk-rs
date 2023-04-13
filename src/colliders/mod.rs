use glam::{DVec3, DMat4, Vec4Swizzles, dvec3, DMat3};

pub mod random;
pub mod support_point;

pub enum ColliderType {
    Sphere,
    Capluse,
    Cylinder,
}

pub struct Collider {
    pub typ: usize,

    pub transform: DMat3,
    pub transform_transposed: DMat3,
    pub center: DVec3,

    pub radius: f64,
    pub height: f64,
}

impl Collider {

    pub fn new_sphere(center: DVec3, radius: f64) -> Self {
        let transform = DMat3::IDENTITY;

        Self { 
            typ: ColliderType::Sphere as usize, 
            transform, 
            transform_transposed: transform.transpose(),
            center: center, 
            radius: radius, 
            height: 0.0,
        }
    }

    pub fn new_capluse(collider2origin: DMat4, radius: f64, height: f64) -> Self {
        let transform = DMat3::from_mat4(collider2origin);

        Self { 
            typ: ColliderType::Capluse as usize, 
            transform, 
            transform_transposed: transform.transpose(),
            center: Self::get_center_from_collider2origin(&collider2origin), 
            radius: radius, 
            height: height,
        }
    }

    pub fn new_cylinder(collider2origin: DMat4, radius: f64, height: f64) -> Self {
        let transform = DMat3::from_mat4(collider2origin);

        Self { 
            typ: ColliderType::Cylinder as usize, 
            transform, 
            transform_transposed: transform.transpose(),
            center: Self::get_center_from_collider2origin(&collider2origin), 
            radius: radius, 
            height: height,
        }
    }

    fn get_center_from_collider2origin(collider2origin: &DMat4 ) -> DVec3 {
        collider2origin.w_axis.xyz()
    }

}