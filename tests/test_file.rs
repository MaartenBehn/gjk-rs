use gjk::{json_loder::load_test_file, gjk::GJKNesterov};

#[test]
fn test_run_test_file() {

    let path = "../data/test_data.json";
    let test_data = load_test_file(path);

    for data in test_data {
        let mut gjk = GJKNesterov::new(None, 1e-6);

        let (_, test_distance) = gjk.intersect_nesterov_accelerated(&data.0, &data.1, 100);

        assert!((test_distance - data.2).abs() < 0.01);
    }
}