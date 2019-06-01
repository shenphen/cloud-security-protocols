extern crate array_tool;
extern crate pairing;

mod garbled_circuit;
mod oblivious_transfer;
mod private_set_intersection;
mod proof_of_possession;
pub mod traits;
pub mod utils;

use garbled_circuit::GarbledCircuit;
use oblivious_transfer::ObliviousTransfer;
use private_set_intersection::PrivateSetIntersection;
use proof_of_possession::ProofOfPossession;
use traits::Protocol;

#[allow(dead_code)]
fn pop() {
    ProofOfPossession::new().set_number_of_blocks(5).run();
}

#[allow(dead_code)]
fn oblivious_transfer() {
    ObliviousTransfer::new().set_n(2).run();
}

#[allow(dead_code)]
fn private_set_intersection() {
    PrivateSetIntersection::new().run();
}

#[allow(dead_code)]
fn garbled_circuit() {
    GarbledCircuit::new().run();
}

fn main() {
    garbled_circuit();
}
