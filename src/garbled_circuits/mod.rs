mod evaluator;
mod garbler;

use super::traits::Protocol;
use evaluator::Evaluator;
use garbler::Garbler;

pub struct GarbledCircuit {}

impl Protocol for GarbledCircuit {
    fn new() -> Self {
        GarbledCircuit {}
    }

    fn protocol(&self) {
        let mut garbler = Garbler::new();
        let mut evaluator = Evaluator::new();

        let garbled_output = garbler.produce_garbled_output();
        let garblers_choice = garbler.get_choise();

        let random_masks = garbler.generate_random_masks();
        let blinded_random_mask = evaluator.choose_random_mask(random_masks);
        let ciphertexts = garbler.generate_ciphertexts(blinded_random_mask);

        let garbled_result =
            evaluator.generate_garbled_result(ciphertexts, garblers_choice, garbled_output);

        println!("Garbled circuit result: {}", garbled_result);
    }
}
