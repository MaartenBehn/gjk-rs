use glam::{Vec3, Mat4, Vec4, Vec4Swizzles};

pub mod random;
pub mod support_point;

enum ColliderType {
    Sphere,
    Capluse,
    Cylinder,
}

pub struct Collider {
    typ: usize,

    collider2origen: Mat4,
    center: Vec3,

    radius: f32,
    height: f32,
}

impl Collider {

    pub fn new_sphere(center: Vec3, radius: f32) -> Self {
        Self { 
            typ: ColliderType::Sphere as usize, 
            collider2origen: Mat4::IDENTITY, 
            
            center: center, 
            radius: radius, 
            height: 0.0,
        }
    }

    pub fn new_capluse(collider2origen: Mat4, radius: f32, height: f32) -> Self {
        Self { 
            typ: ColliderType::Capluse as usize, 
            collider2origen: collider2origen, 
            
            center: collider2origen.w_axis.xyz(), 
            radius: radius, 
            height: height,
        }
    }

    pub fn new_cylinder(collider2origen: Mat4, radius: f32, height: f32) -> Self {
        Self { 
            typ: ColliderType::Cylinder as usize, 
            collider2origen: collider2origen, 
            
            center: collider2origen.w_axis.xyz(), 
            radius: radius, 
            height: height,
        }
    }

}

