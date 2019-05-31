extern crate pairing;

mod oblivious_transfer;
mod proof_of_possession;
pub mod traits;
pub mod utils;

use oblivious_transfer::ObliviousTransfer;
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

fn main() {
    oblivious_transfer();
}
