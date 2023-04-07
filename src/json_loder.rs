use std::fs;

use glam::{vec3, Mat4};
use json::JsonValue;

use crate::colliders::Collider;

pub fn load_test_file(path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let json_data = json::parse(&contents).unwrap();

    let mut i = 0;
    for json_obj in json_data.members(){
        println!("Interation: {:?}", i);

        let collider0 = parse_collider(&json_obj["collider1"]);
        let collider1 = parse_collider(&json_obj["collider2"]);

        let distance = json_obj["distance"].as_f32().unwrap();


        i += 1;
    }
}

fn parse_collider(json_obj: &JsonValue) -> Collider {
    match json_obj["type"].as_str().unwrap() {
        "Sphere" => {
            let center = vec3(
                json_obj["center"][0].as_f32().unwrap(),
                json_obj["center"][1].as_f32().unwrap(),
                json_obj["center"][2].as_f32().unwrap(),
            );

            let radius = json_obj["radius"].as_f32().unwrap();

            Collider::new_sphere(center, radius)
        }
        "Capsule" => {
            let radius = json_obj["radius"].as_f32().unwrap();
            let height = json_obj["height"].as_f32().unwrap();

            Collider::new_capluse(Mat4::IDENTITY, radius, height)
        }   
        "Cylinder" => {
            let radius = json_obj["radius"].as_f32().unwrap();
            let height = json_obj["height"].as_f32().unwrap();

            Collider::new_cylinder(Mat4::IDENTITY, radius, height)
        }
        &_ => todo!()
    }
}