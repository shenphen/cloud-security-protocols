use pairing::bls12_381::{Fr, G1};
use pairing::CurveProjective;
use pairing::Field;
use rand::Rand;

pub struct Client {
    n: usize,
    messages: Vec<G1>,
    random_exponents: Vec<Fr>,
}

impl Client {
    pub fn new(n: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut messages: Vec<G1> = vec![];

        for _ in 0..n {
            let m = G1::rand(&mut rng);
            messages.push(m);
        }

        Client {
            n,
            messages,
            random_exponents: vec![],
        }
    }

    pub fn generate_random_masks(&mut self) -> Vec<G1> {
        let mut rng = rand::thread_rng();
        self.random_exponents = vec![];
        let mut random_masks: Vec<G1> = vec![];

        for _ in 0..self.n {
            let r = Fr::rand(&mut rng);
            self.random_exponents.push(r);
            let mut random_mask = G1::one();
            random_mask.mul_assign(r);
            random_masks.push(random_mask);
        }

        random_masks
    }

    pub fn generate_ciphertexts(&self, random_mask: G1) -> Vec<G1> {
        let mut ciphertexts: Vec<G1> = vec![];

        for i in 0..self.n {
            let mut w = random_mask;
            let r = self.random_exponents[i];
            let r_inv = match r.inverse() {
                Some(x) => x,
                None => Fr::one(),
            };
            w.mul_assign(r_inv);
            w.add_assign(&self.messages[i]);
            ciphertexts.push(w);
        }

        ciphertexts
    }

    pub fn verify_message(&self, m: G1, index: usize) -> bool {
        if index >= self.messages.len() {
            return false;
        }

        self.messages[index] == m
    }
}
