use gjk::colliders::Collider;
use rand::{rngs::StdRng, SeedableRng};


#[test]
fn test_support_functions(){

    let mut rng = StdRng::seed_from_u64(42);
    let size_range = 0.0..100.0;

    for i in 0..100 {
        let collider0 = Collider::new_random(&mut rng, size_range.to_owned());
        let collider1 = Collider::new_random(&mut rng, size_range.to_owned());

        println!("{:?}", i)
    } 
}