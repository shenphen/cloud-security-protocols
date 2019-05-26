extern crate pairing;

mod client;
mod cloud;
mod helpers;
pub mod types;

use client::Client;
use cloud::Cloud;
use helpers::PerformanceTimer;
use pairing::bls12_381::Fr;
use rand::Rand;
use types::{File, IdFile};

const NUMBER_OF_BLOCKS: usize = 10;

fn protocol() {
    let mut rng = rand::thread_rng();

    let client = Client::new(NUMBER_OF_BLOCKS);
    let mut cloud = Cloud::new();

    let mut file: File = vec![];
    let id_file: IdFile = 1;

    for _ in 0..NUMBER_OF_BLOCKS {
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

fn main() {
    protocol();
}
