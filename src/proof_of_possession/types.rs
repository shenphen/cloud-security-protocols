use pairing::bls12_381::{Fr, G1};

pub type IdFile = usize;
pub type TaggedFile = Vec<(Fr, Fr)>;
pub type File = Vec<Fr>;
pub type Challange = (G1, Fr, G1);
