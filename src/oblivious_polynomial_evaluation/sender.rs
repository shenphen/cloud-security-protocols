use super::super::utils::{eval_poly_at_point, PolynomialRepr};
use pairing::bls12_381::Fr;
use pairing::Field;
use rand::Rand;

pub struct Sender {
    polynomial_repr: PolynomialRepr,
    masking_polynomial_repr: PolynomialRepr,
}

impl Sender {
    pub fn new(k: usize, d_p: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut polynomial_repr: PolynomialRepr = vec![];
        let mut masking_polynomial_repr: PolynomialRepr = vec![];

        for i in 0..(d_p + 1) {
            let b_i = Fr::rand(&mut rng);
            polynomial_repr.push((b_i, i));
        }

        for i in 0..(k * d_p + 1) {
            let b_i = if i == 0 {
                Fr::zero()
            } else {
                Fr::rand(&mut rng)
            };
            masking_polynomial_repr.push((b_i, i));
        }

        Sender {
            polynomial_repr,
            masking_polynomial_repr,
        }
    }

    pub fn eval_bivariate_poly(&self, x: &Fr, y: &Fr) -> Fr {
        let mut result = eval_poly_at_point(&self.masking_polynomial_repr, x);
        result.add_assign(&eval_poly_at_point(&self.polynomial_repr, y));

        result
    }

    pub fn verify_calculation(&self, alpha: Fr, computed_result: Fr) -> bool {
        eval_poly_at_point(&self.polynomial_repr, &alpha) == computed_result
    }
}
