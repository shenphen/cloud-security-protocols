use pairing::bls12_381::{Fr, G1};
use pairing::CurveProjective;
use pairing::Field;
use rand::Rand;

pub struct Client {
    set: Vec<G1>,
    ephemeral: Fr,
}

impl Client {
    pub fn new(intersect_values: &Vec<G1>, additional_random_amount: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut set: Vec<G1> = vec![];

        for _ in 0..additional_random_amount {
            let random = G1::rand(&mut rng);
            set.push(random);
        }

        set.extend(intersect_values);

        Client {
            set,
            ephemeral: Fr::zero(),
        }
    }

    pub fn send_masked_set(&mut self) -> Vec<G1> {
        let mut rng = rand::thread_rng();
        self.ephemeral = Fr::rand(&mut rng);
        let mut masked_set = vec![];

        for i in 0..self.set.len() {
            let mut masked_value = self.set[i];
            masked_value.mul_assign(self.ephemeral);
            masked_set.push(masked_value);
        }

        masked_set
    }

    pub fn encrypt_masked_set(&self, masked_set: Vec<G1>) -> Vec<G1> {
        let mut ciphertexts: Vec<G1> = vec![];

        for i in 0..masked_set.len() {
            let mut ciphertext = masked_set[i];
            ciphertext.mul_assign(self.ephemeral);
            ciphertexts.push(ciphertext);
        }

        ciphertexts
    }

    pub fn get_ephemeral(&self) -> Fr {
        self.ephemeral
    }
}
