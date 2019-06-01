use super::super::utils::hash_points;
use pairing::bls12_381::{Fr, G1};
use pairing::{CurveProjective, Field};
use rand::{Rand, Rng};

pub struct Evaluator {
    chosen_index: usize,
    alpha: Fr,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            chosen_index: 0,
            alpha: Fr::one(),
        }
    }

    pub fn choose_random_mask(&mut self, random_masks: [G1; 2]) -> G1 {
        let mut rng = rand::thread_rng();
        self.alpha = Fr::rand(&mut rng);
        self.chosen_index = rng.gen_range(0, random_masks.len());
        println!("Evaluator's choice: {}", self.chosen_index);

        let mut blinded_random_mask = random_masks[self.chosen_index];
        blinded_random_mask.mul_assign(self.alpha);

        blinded_random_mask
    }

    pub fn generate_garbled_result(
        &self,
        ciphertexts: [G1; 2],
        garblers_choice: G1,
        garbled_output: Vec<(u64, usize)>,
    ) -> usize {
        let mut evaluators_choice = ciphertexts[self.chosen_index];
        let mut denominator = G1::one();
        denominator.mul_assign(self.alpha);
        evaluators_choice.sub_assign(&denominator);

        let hash = hash_points(garblers_choice, evaluators_choice);
        let index = garbled_output.iter().position(|&r| r.0 == hash).unwrap();

        garbled_output[index].1
    }
}
