use std::fs;

use glam::{dvec3, DMat4, dmat4, dvec4, DVec3, DVec4};
use json::JsonValue;

use crate::{colliders::Collider, gjk::GJKNesterov};

pub fn run_test_file(path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let json_data = json::parse(&contents).unwrap();

    let mut i = 0;
    for json_obj in json_data.members(){
        println!("Interation: {:?}", i);

        let collider1 = parse_collider(&json_obj["collider1"]);
        let collider2 = parse_collider(&json_obj["collider2"]);

        let distance = json_obj["distance"].as_f32().unwrap();

        let mut gjk = GJKNesterov::new(None, 1e-6);
        let (collide, test_distance) = gjk.intersect_nesterov_accelerated(collider1, collider2, 100);

        assert!(collide == (distance == 0.0));
        //assert!(test_distance == distance);

        i += 1;
    }
}

pub fn parse_collider(json_obj: &JsonValue) -> Collider {
    match json_obj["type"].as_str().unwrap() {
        "Sphere" => {
            let center = parse_vec3(&json_obj["center"]);

            let radius = json_obj["radius"].as_f64().unwrap();

            Collider::new_sphere(center, radius)
        }
        "Capsule" => {
            let collider2origin = parse_mat4(&json_obj["collider2origin"]);

            let radius = json_obj["radius"].as_f64().unwrap();
            let height = json_obj["height"].as_f64().unwrap();

            Collider::new_capluse(collider2origin, radius, height)
        }   
        "Cylinder" => {
            let collider2origin = parse_mat4(&json_obj["collider2origin"]);

            let radius = json_obj["radius"].as_f64().unwrap();
            let height = json_obj["height"].as_f64().unwrap();

            Collider::new_cylinder(collider2origin, radius, height)
        }
        &_ => todo!()
    }
}

fn parse_vec3(json_obj: &JsonValue) -> DVec3 {
    dvec3(
        json_obj[0].as_f64().unwrap(),
        json_obj[1].as_f64().unwrap(),
        json_obj[2].as_f64().unwrap(),
    )
}

fn parse_vec4(json_obj: &JsonValue) -> DVec4 {
    dvec4(
        json_obj[0].as_f64().unwrap(),
        json_obj[1].as_f64().unwrap(),
        json_obj[2].as_f64().unwrap(),
        json_obj[3].as_f64().unwrap(),
    )
}

fn parse_mat4(json_obj: &JsonValue) -> DMat4 {
    dmat4(
        parse_vec4(&json_obj[0]),
        parse_vec4(&json_obj[1]),
        parse_vec4(&json_obj[2]),
        parse_vec4(&json_obj[3]),
    )
}

#[cfg(test)]
mod test{
    use glam::dvec3;

    use crate::{colliders::ColliderType, json_loder::parse_collider};

    use super::run_test_file;

    #[test]
    fn test_run_test_file() {

        let path = "../data/test_data.json";
        run_test_file(path);
    }

    #[test]
    fn test_parse_json_collider() {
        let json_obj = json::parse(r#"
        {
            "type": "Sphere",
            "center": [
                1.0,
                0.0,
                0.0
            ],
            "radius": 10.0
        }"#).unwrap();

        let collider = parse_collider(&json_obj);
        assert!(collider.typ == (ColliderType::Sphere as usize));
        assert!(collider.center == dvec3(1.0, 0.0, 0.0));
        assert!(collider.radius == 10.0);


        let json_obj = json::parse(r#"
        {
            "type": "Capsule",
            "center": [
                0.0,
                1.0,
                2.0
            ],
            "radius": 1.0,
            "height": 2.0
        }"#).unwrap();

        let collider = parse_collider(&json_obj);
        assert!(collider.typ == (ColliderType::Capluse as usize));
        // assert!(collider.center == vec3(0.0, 1.0, 2.0)); TODO
        assert!(collider.radius == 1.0);
        assert!(collider.height == 2.0);


        let json_obj = json::parse(r#"
        {
            "type": "Cylinder",
            "center": [
                5.0,
                1.0,
                0.0
            ],
            "radius": 10.0,
            "height": 3.0
        }"#).unwrap();

        let collider = parse_collider(&json_obj);
        assert!(collider.typ == (ColliderType::Cylinder as usize));
        // assert!(collider.center == vec3(5.0, 1.0, 0.0)); TODO
        assert!(collider.radius == 10.0);
        assert!(collider.height == 3.0);
    }
}

