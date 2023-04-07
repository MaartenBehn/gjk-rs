use glam::{Vec3, Vec4, Vec4Swizzles};

use super::{Collider, ColliderType};


impl Collider {
    pub fn get_support_point(&self, dir: Vec3) -> Vec3 {
        match self.typ {
            x if x == ColliderType::Sphere as usize => {
                dir.normalize() * self.radius + self.center
            },
    
            x if x == ColliderType::Capluse as usize => {
                /*
                You can find similar implementations here:
    
                * https://github.com/kevinmoran/GJK/blob/b38d923d268629f30b44c3cf6d4f9974bbcdb0d3/Collider.h#L57
                (Copyright (c) 2017 Kevin Moran, MIT License or Unlicense)
                * https://github.com/bulletphysics/bullet3/blob/e306b274f1885f32b7e9d65062aa942b398805c2/src/BulletCollision/CollisionShapes/btConvexShape.cpp#L228
                (Copyright (c) 2003-2009 Erwin Coumans, zlib license)
                */
    
                let local_dir = self.collider2origen.inverse() * Vec4::from((dir, 0.0));
    
                let mut local_vertex = local_dir.normalize() * self.radius;
    
                local_vertex.z += if local_vertex.z > 0.0 {
                    0.5 * self.height
                } else {
                    -0.5 * self.height
                };
    
                (self.collider2origen * local_vertex).xyz() + self.center
            },
    
            x if x == ColliderType::Cylinder as usize => {
                /*
                You can find similar implementations here:
    
                * https://github.com/kevinmoran/GJK/blob/b38d923d268629f30b44c3cf6d4f9974bbcdb0d3/Collider.h#L42
                (Copyright (c) 2017 Kevin Moran, MIT License or Unlicense)
                * https://github.com/bulletphysics/bullet3/blob/e306b274f1885f32b7e9d65062aa942b398805c2/src/BulletCollision/CollisionShapes/btConvexShape.cpp#L167
                (Copyright (c) 2003-2009 Erwin Coumans, zlib license) 
                */
    
                let local_dir = self.collider2origen.inverse() * Vec4::from((dir, 0.0));
    
                let mut local_vertex = local_dir.normalize() * self.radius;
    
                local_vertex.z = 0.0;
    
                local_vertex.z += if local_vertex.z > 0.0 {
                    0.5 * self.height
                } else {
                    -0.5 * self.height
                };
    
                (self.collider2origen * local_vertex).xyz() + self.center
            },    
            _ => todo!() 
        }
    }
    
    pub fn get_support_point_table(&self, dir: Vec3) -> Vec3 {

        const PERFORM_Z_OFFSET_TABLE: [f32; 3] = [0., 1., 1.];
        const ADD_TO_Z_TABLE: [f32; 3] = [1., 1., 0.];

        let local_dir = self.collider2origen.inverse() * Vec4::from((dir, 0.0));
    
        let mut local_vertex = local_dir.normalize() * self.radius;
        
        let z_offset = if local_vertex.z > 0.0 {
            0.5 * self.height
        } else {
            -0.5 * self.height
        };
    
        local_vertex.z = local_vertex.z * ADD_TO_Z_TABLE[self.typ] + z_offset * PERFORM_Z_OFFSET_TABLE[self.typ];
    
        (self.collider2origen * local_vertex).xyz() + self.center
    }
}