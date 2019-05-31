mod client;

use super::traits::Protocol;
use array_tool::vec::Intersect;
use client::Client;
use pairing::bls12_381::G1;
use pairing::CurveProjective;
use rand::Rand;

const INTERSECT_AMOUNT: usize = 2;

pub struct PrivateSetIntersection {}

impl Protocol for PrivateSetIntersection {
    fn new() -> Self {
        PrivateSetIntersection {}
    }

    fn protocol(&self) {
        let mut rng = rand::thread_rng();
        let mut intersect_values: Vec<G1> = vec![];

        for _ in 0..INTERSECT_AMOUNT {
            let random = G1::rand(&mut rng);
            intersect_values.push(random);
        }

        let mut client_a = Client::new(&intersect_values, 3);
        let mut client_b = Client::new(&intersect_values, 2);

        let masked_set_a = client_a.send_masked_set();
        let masked_set_b = client_b.send_masked_set();

        let ciphertexts_a = client_a.encrypt_masked_set(masked_set_b);
        let ciphertexts_b = client_b.encrypt_masked_set(masked_set_a);

        let intersect_ciphertexts: Vec<G1> = ciphertexts_a.intersect(ciphertexts_b);
        println!(
            "Intersect ciphertexts length: {}",
            intersect_ciphertexts.len()
        );

        let ephemeral_a = client_a.get_ephemeral();
        let ephemeral_b = client_b.get_ephemeral();

        for i in 0..INTERSECT_AMOUNT {
            let mut intersect_value = intersect_values[i];
            intersect_value.mul_assign(ephemeral_a);
            intersect_value.mul_assign(ephemeral_b);
            println!(
                "Verification result: {}",
                intersect_ciphertexts.contains(&intersect_value)
            );
        }
    }
}
