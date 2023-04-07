use glam::{vec3, Vec3, Mat4};
use rand::{Rng, distributions::uniform::SampleRange};

use super::Collider;


impl Collider {
    pub fn new_random<R: SampleRange<f32> + Clone>(rng: &mut impl Rng, size_range: R) -> Self {
        match rng.gen_range(0..=2) {
            // rand 0.8
            0 => Self::new_random_sphere(rng, size_range.to_owned(), size_range.to_owned()),
            1 => Self::new_random_capsule(rng, size_range.to_owned(), size_range.to_owned(), size_range.to_owned()),
            2 => Self::new_random_cylinder(rng, size_range.to_owned(), size_range.to_owned(), size_range.to_owned()),
            _ => todo!()
        }
    }

    pub fn new_random_sphere<R: SampleRange<f32> + Clone>(
        rng: &mut impl Rng, 
        center_rang: R,
        radius_range: R) -> Self
    {
        Self::new_sphere(
            random_vec3(rng, center_rang), 
            rng.gen_range(radius_range))
    }

    pub fn new_random_capsule<R: SampleRange<f32> + Clone>(
        rng: &mut impl Rng, 
        center_rang: R,
        radius_range: R,
        height_range: R) -> Self
    {
        Self::new_capluse(
            random_mat4(rng, center_rang), 
            rng.gen_range(radius_range), 
            rng.gen_range(height_range))
    }

    pub fn new_random_cylinder<R: SampleRange<f32> + Clone>(
        rng: &mut impl Rng, 
        center_rang: R,
        radius_range: R,
        height_range: R) -> Self
    {
        Self::new_cylinder(
            random_mat4(rng, center_rang), 
            rng.gen_range(radius_range), 
            rng.gen_range(height_range))
    }
}

fn random_vec3<R: SampleRange<f32> + Clone>(rng: &mut impl Rng, range: R) -> Vec3 {
    vec3(
        rng.gen_range(range.to_owned()), 
        rng.gen_range(range.to_owned()), 
        rng.gen_range(range.to_owned()))
}

fn random_mat4<R: SampleRange<f32> + Clone>(rng: &mut impl Rng, range: R) -> Mat4 {
    Mat4::from_translation(random_vec3(rng, range))
}