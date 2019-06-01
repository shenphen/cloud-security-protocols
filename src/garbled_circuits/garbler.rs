use super::super::utils::hash_points;
use pairing::bls12_381::{Fr, G1};
use pairing::{CurveProjective, Field};
use rand::{Rand, Rng};

pub struct Garbler {
    w_g: [G1; 2],
    w_e: [G1; 2],
    random_exponents: [Fr; 2],
}

impl Garbler {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let w_g_0 = G1::rand(&mut rng);
        let w_g_1 = G1::rand(&mut rng);
        let w_e_0 = G1::rand(&mut rng);
        let w_e_1 = G1::rand(&mut rng);

        Garbler {
            w_g: [w_g_0, w_g_1],
            w_e: [w_e_0, w_e_1],
            random_exponents: [Fr::rand(&mut rng), Fr::rand(&mut rng)],
        }
    }

    pub fn produce_garbled_output(&self) -> Vec<(u64, usize)> {
        let mut rng = rand::thread_rng();
        let mut garbled_output = vec![];

        for i in 0..2 {
            for j in 0..2 {
                let hash = hash_points(self.w_g[i], self.w_e[j]);
                garbled_output.insert(2 * i + j, (hash, i * j));
            }
        }
        rng.shuffle(&mut garbled_output);

        garbled_output
    }

    pub fn get_choise(&self) -> G1 {
        let mut rng = rand::thread_rng();
        let choice: usize = rng.gen_range(0, 2);
        println!("Garbler's choise: {}", choice);

        self.w_g[choice]
    }

    pub fn generate_random_masks(&mut self) -> [G1; 2] {
        let mut rng = rand::thread_rng();
        let mut random_masks: [G1; 2] = [G1::one(); 2];

        for i in 0..2 {
            let r = Fr::rand(&mut rng);
            self.random_exponents[i] = r;
            let mut random_mask = G1::one();
            random_mask.mul_assign(r);
            random_masks[i] = random_mask;
        }

        random_masks
    }

    pub fn generate_ciphertexts(&self, random_mask: G1) -> [G1; 2] {
        let mut ciphertexts: [G1; 2] = [G1::one(); 2];

        for i in 0..2 {
            let mut w = random_mask;
            let r = self.random_exponents[i];
            let r_inv = match r.inverse() {
                Some(x) => x,
                None => Fr::one(),
            };
            w.mul_assign(r_inv);
            w.add_assign(&self.w_e[i]);
            ciphertexts[i] = w;
        }

        ciphertexts
    }
}
