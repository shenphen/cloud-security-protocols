mod client;
mod cloud;

use super::traits::Protocol;
use client::Client;
use cloud::Cloud;

pub struct ObliviousTransfer {
    n: usize,
}

impl ObliviousTransfer {
    pub fn set_n(mut self, n: usize) -> Self {
        self.n = n;
        self
    }
}

impl Protocol for ObliviousTransfer {
    fn new() -> Self {
        ObliviousTransfer { n: 10 }
    }

    fn protocol(&self) {
        let mut client = Client::new(self.n);
        let mut cloud = Cloud::new();

        let random_masks = client.generate_random_masks();
        let (blinded_random_mask, k) = cloud.choose_random_mask(random_masks);

        let ciphertexts = client.generate_ciphertexts(blinded_random_mask);
        let message = cloud.get_message(ciphertexts);

        println!("Verification result: {}", client.verify_message(message, k));
    }
}
