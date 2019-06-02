use pairing::bls12_381::{Fr, G1Affine, G1Compressed, G1};
use pairing::{EncodedPoint, Field};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::time::Instant;

pub struct PerformanceTimer {
    instant: Instant,
    title: String,
    is_measuring: bool,
}

impl PerformanceTimer {
    pub fn new() -> Self {
        PerformanceTimer {
            instant: Instant::now(),
            title: "PerformanceTimerInit".to_owned(),
            is_measuring: false,
        }
    }

    pub fn start(&mut self, title: String) {
        if self.is_measuring {
            self.stop();
        }

        self.is_measuring = true;
        self.title = title;
        self.instant = Instant::now();
    }

    pub fn stop(&mut self) {
        if self.is_measuring {
            self.is_measuring = false;
            let elapsed = self.instant.elapsed();
            let sec = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1000_000_000.0);
            println!("{} took {} seconds", self.title, sec);
        } else {
            println!("No active timer is run");
        }
    }
}

pub fn hash_points(p1: G1, p2: G1) -> u64 {
    let p1_affine = G1Affine::from(p1);
    let p2_affine = G1Affine::from(p2);
    let p1_compressed = G1Compressed::from_affine(p1_affine);
    let p2_compressed = G1Compressed::from_affine(p2_affine);

    let mut hasher = DefaultHasher::new();
    let vec = [p1_compressed.as_ref(), p2_compressed.as_ref()].concat();
    for val in vec {
        hasher.write_u8(val);
    }

    hasher.finish()
}

pub type PolynomialRepr = Vec<(Fr, usize)>;

pub fn eval_poly_at_point(poly: &PolynomialRepr, x: &Fr) -> Fr {
    let mut result = Fr::zero();

    for i in 0..poly.len() {
        let mut coefficient = Fr::one();
        let a_i = poly[i].0;
        let power = poly[i].1 as u64;
        coefficient.mul_assign(&a_i);
        let x_pow = x.pow([power]);
        if power == 0 {
            assert_eq!(x_pow, Fr::one());
        }
        coefficient.mul_assign(&x_pow);
        result.add_assign(&coefficient);
    }

    result
}
