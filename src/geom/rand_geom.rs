use crate::types::{Vector3f};
use rand::Rng;

pub fn random_in_unit_sphere() -> Vector3f {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-1.0, 1.0);
    let y = rng.gen_range(-1.0, 1.0);
    let z = rng.gen_range(-1.0, 1.0);
    Vector3f::new(x, y, z).normalize()
}
