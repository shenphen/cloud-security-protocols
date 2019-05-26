use pairing::bls12_381::{Fr, G1};

pub type IdFile = usize;
pub type TaggedFile = Vec<(Fr, Fr)>;
pub type File = Vec<Fr>;
pub type Polynomial = Vec<(Fr, usize)>;
pub type Challange = (G1, Fr, G1);
