mod receiver;
mod sender;

use super::traits::Protocol;
use pairing::bls12_381::Fr;
use pairing::Field;
use rand::Rand;
use receiver::Receiver;
use sender::Sender;
use std::process::exit;

pub struct ObliviousPolynomialEvaluation {
    k: usize,
    d_p: usize,
}

impl ObliviousPolynomialEvaluation {
    pub fn set_security_parameter(mut self, k: usize) -> Self {
        self.k = k;
        self
    }
    pub fn set_polynomial_degree(mut self, d_p: usize) -> Self {
        self.d_p = d_p;
        self
    }
}

impl Protocol for ObliviousPolynomialEvaluation {
    fn new() -> Self {
        ObliviousPolynomialEvaluation { k: 10, d_p: 5 }
    }

    fn protocol(&self) {
        if self.d_p >= self.k {
            println!("Security parameter k has to be greater than polynomial degree in order to protect receiver.");
            exit(1);
        }

        let mut a = Fr::zero();
        a.add_assign(&Fr::one());

        assert_eq!(a, Fr::one());

        let mut rng = rand::thread_rng();

        let sender = Sender::new(self.k, self.d_p);
        let receiver = Receiver::new(self.k);

        let mut points_of_univariate_polynomial: Vec<(Fr, Fr)> = vec![];
        for _ in 0..(self.k * self.d_p + 1) {
            let x = Fr::rand(&mut rng);
            let s_x = receiver.evaluate_random_poly(&x);
            let r_x = sender.eval_bivariate_poly(&x, &s_x);
            assert_eq!(points_of_univariate_polynomial.contains(&(x, r_x)), false);

            points_of_univariate_polynomial.push((x, r_x));
        }

        let p_x = receiver.interpolate(points_of_univariate_polynomial, Fr::zero());
        let alpha = receiver.reveal_alpha();

        println!(
            "Verification result: {}",
            sender.verify_calculation(alpha, p_x)
        );
    }
}
