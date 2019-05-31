use pairing::bls12_381::{Fr, G1};
use pairing::CurveProjective;
use pairing::Field;
use rand::{Rand, Rng};

pub struct Cloud {
    chosen_index: usize,
    alpha: Fr,
}

impl Cloud {
    pub fn new() -> Self {
        Cloud {
            chosen_index: 0,
            alpha: Fr::one(),
        }
    }

    pub fn choose_random_mask(&mut self, random_masks: Vec<G1>) -> (G1, usize) {
        let mut rng = rand::thread_rng();
        self.alpha = Fr::rand(&mut rng);
        self.chosen_index = rng.gen_range(0, random_masks.len());
        println!("Chosen index: {}", self.chosen_index);

        let mut blinded_random_mask = random_masks[self.chosen_index];
        blinded_random_mask.mul_assign(self.alpha);

        (blinded_random_mask, self.chosen_index)
    }

    pub fn get_message(&self, ciphertexts: Vec<G1>) -> G1 {
        if self.chosen_index >= ciphertexts.len() {
            return G1::zero();
        }

        let mut m = ciphertexts[self.chosen_index];
        let mut denominator = G1::one();
        denominator.mul_assign(self.alpha);
        m.sub_assign(&denominator);

        m
    }
}
