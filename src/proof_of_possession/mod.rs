mod client;
mod cloud;
pub mod types;

use super::traits::Protocol;
use super::utils::PerformanceTimer;
use client::Client;
use cloud::Cloud;
use pairing::bls12_381::Fr;
use rand::Rand;
use types::{File, IdFile};

pub struct ProofOfPossession {
    number_of_blocks: usize,
}

impl ProofOfPossession {
    pub fn set_number_of_blocks(mut self, number_of_blocks: usize) -> Self {
        self.number_of_blocks = number_of_blocks;
        self
    }
}

impl Protocol for ProofOfPossession {
    fn new() -> Self {
        ProofOfPossession {
            number_of_blocks: 10,
        }
    }

    fn protocol(&self) {
        let mut rng = rand::thread_rng();

        let client = Client::new(self.number_of_blocks);
        let mut cloud = Cloud::new();

        let mut file: File = vec![];
        let id_file: IdFile = 1;

        for _ in 0..self.number_of_blocks {
            file.push(Fr::rand(&mut rng));
        }

        let mut timer = PerformanceTimer::new();
        timer.start("Protocol execution".to_owned());

        let tagged_file = client.tag_block(file, id_file);
        cloud.add_file(id_file, tagged_file);

        let (k_f, challange) = client.gen_challange(id_file);

        let p_f = cloud.gen_proof(id_file, challange);

        timer.stop();

        println!("Kf == Pf: {}", k_f == p_f);
        println!("Kf: {}", k_f);
        println!("Pf: {}", p_f);
    }
}
