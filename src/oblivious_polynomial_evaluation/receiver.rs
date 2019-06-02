use super::super::utils::{eval_poly_at_point, PolynomialRepr};
use pairing::bls12_381::Fr;
use pairing::Field;
use rand::Rand;

pub struct Receiver {
    alpha: Fr,
    random_polynomial_repr: PolynomialRepr,
}

impl Receiver {
    pub fn new(k: usize) -> Self {
        let mut rng = rand::thread_rng();
        let alpha = Fr::rand(&mut rng);
        let mut random_polynomial_repr: PolynomialRepr = vec![];

        for i in 0..(k + 1) {
            let b_i = if i == 0 { alpha } else { Fr::rand(&mut rng) };
            random_polynomial_repr.push((b_i, i));
        }

        assert_eq!(
            eval_poly_at_point(&random_polynomial_repr, &Fr::zero()),
            alpha
        );

        Receiver {
            alpha,
            random_polynomial_repr,
        }
    }

    pub fn evaluate_random_poly(&self, x: &Fr) -> Fr {
        eval_poly_at_point(&self.random_polynomial_repr, x)
    }

    pub fn interpolate(&self, points: Vec<(Fr, Fr)>, x: Fr) -> Fr {
        let mut result: Fr = Fr::zero();
        let n = points.len();

        for i in 0..n {
            let mut term: Fr = points[i].1;
            for j in 0..n {
                if j != i {
                    let mut term_multiplier = x;
                    term_multiplier.sub_assign(&points[j].0);

                    let mut denominator = points[i].0;
                    denominator.sub_assign(&points[j].0);
                    let inv = match denominator.inverse() {
                        Some(inv) => inv,
                        None => {
                            println!("Couldn't make inverse");
                            Fr::one()
                        }
                    };

                    term_multiplier.mul_assign(&inv);
                    term.mul_assign(&term_multiplier);
                }
            }

            result.add_assign(&term);
        }

        result
    }

    pub fn reveal_alpha(&self) -> Fr {
        self.alpha
    }
}
