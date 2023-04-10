use glam::{DVec3, DMat4, Vec4Swizzles, dvec3};

pub mod random;
pub mod support_point;

pub enum ColliderType {
    Sphere,
    Capluse,
    Cylinder,
}

pub struct Collider {
    pub typ: usize,

    pub collider2origin: DMat4,
    pub center: DVec3,

    pub radius: f64,
    pub height: f64,
}

impl Collider {

    pub fn new_sphere(center: DVec3, radius: f64) -> Self {
        Self { 
            typ: ColliderType::Sphere as usize, 
            collider2origin: DMat4::IDENTITY, 
            
            center: center, 
            radius: radius, 
            height: 0.0,
        }
    }

    pub fn new_capluse(collider2origin: DMat4, radius: f64, height: f64) -> Self {
        Self { 
            typ: ColliderType::Capluse as usize, 
            collider2origin, 
            
            center: dvec3(collider2origin.x_axis.w, collider2origin.y_axis.w, collider2origin.z_axis.w), 
            radius: radius, 
            height: height,
        }
    }

    pub fn new_cylinder(collider2origin: DMat4, radius: f64, height: f64) -> Self {
        Self { 
            typ: ColliderType::Cylinder as usize, 
            collider2origin, 
            
            center: dvec3(collider2origin.x_axis.w, collider2origin.y_axis.w, collider2origin.z_axis.w), 
            radius: radius, 
            height: height,
        }
    }

}