use pairing::bls12_381::{Fr, G1};
use pairing::{CurveProjective, Field};
use std::collections::HashMap;

use super::types::{Challange, IdFile, TaggedFile};

pub struct Cloud {
    files: HashMap<IdFile, TaggedFile>,
}

impl Cloud {
    pub fn new() -> Self {
        let files = HashMap::new();

        Cloud { files }
    }

    pub fn add_file(&mut self, id_file: IdFile, tagged_file: TaggedFile) {
        self.files.insert(id_file, tagged_file);
    }

    pub fn gen_proof(&self, id_file: IdFile, (g_r, x_c, g_r_0): Challange) -> G1 {
        let proof = match self.files.get(&id_file) {
            Some(tagged_file) => {
                let mut psi: Vec<(Fr, G1)> = vec![];
                for &(m_i, t_i) in tagged_file {
                    let mut g_rt = g_r;
                    g_rt.mul_assign(t_i);
                    psi.push((m_i, g_rt));
                }
                psi.push((Fr::zero(), g_r_0));

                self.li_exp(psi, x_c)
            }
            None => G1::zero(),
        };

        proof
    }

    fn li_exp(&self, psi: Vec<(Fr, G1)>, x_c: Fr) -> G1 {
        let mut result: G1 = G1::zero();
        let z = psi.len();

        for i in 0..z {
            let mut term: G1 = psi[i].1;
            let mut exponent: Fr = Fr::one();
            for j in 0..z {
                if j != i {
                    let x_i = psi[i].0;
                    let x_j = psi[j].0;
                    let mut denominator = x_i;
                    let mut current_exp_val = x_c;
                    current_exp_val.sub_assign(&x_j);
                    denominator.sub_assign(&x_j);

                    let inv = match denominator.inverse() {
                        Some(inv) => inv,
                        None => {
                            println!("Couldn't make inverse");
                            Fr::one()
                        }
                    };
                    current_exp_val.mul_assign(&inv);
                    exponent.mul_assign(&current_exp_val);
                }
            }

            term.mul_assign(exponent);

            result.add_assign(&term);
        }

        result
    }
}
