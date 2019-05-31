use super::types::{Challange, File, IdFile, Polynomial, TaggedFile};
use pairing::bls12_381::{Fr, G1};
use pairing::CurveProjective;
use pairing::{Field, PrimeField};
use rand::{Rand, SeedableRng, StdRng};

pub struct Client {
    sk: Fr,
    pub z: usize,
}

impl Client {
    pub fn new(z: usize) -> Self {
        let mut rng = rand::thread_rng();
        let sk = Fr::rand(&mut rng);

        Client { sk, z }
    }

    pub fn tag_block(&self, file: File, id_file: IdFile) -> TaggedFile {
        let poly = self.polynomial(id_file);
        let mut tag_block: TaggedFile = vec![];

        for m in file {
            tag_block.push((m, self.get_tag(&poly, &m)));
        }

        tag_block
    }

    fn polynomial(&self, id_file: IdFile) -> Polynomial {
        let mut poly: Polynomial = vec![];
        let sk = self.sk.into_repr().0;
        let seed: &[usize] = &[sk[0] as usize, id_file];
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        for i in 0..self.z {
            let a_i = Fr::rand(&mut rng);
            poly.push((a_i, i));
        }

        poly
    }

    fn get_tag(&self, poly: &Polynomial, x: &Fr) -> Fr {
        let mut tag = Fr::zero();

        for i in 0..self.z {
            let mut coefficient = Fr::one();
            let a_i = poly[i].0;
            let power = poly[i].1 as u64;
            coefficient.mul_assign(&a_i);
            x.pow([power]);
            coefficient.mul_assign(&x);
            tag.add_assign(&coefficient);
        }

        tag
    }

    pub fn gen_challange(&self, id_file: IdFile) -> (G1, Challange) {
        let poly = self.polynomial(id_file);
        let mut rng = rand::thread_rng();
        let r = Fr::rand(&mut rng);
        let x_c = Fr::rand(&mut rng);
        let mut k_exponent = r;
        let mut challange_exponent = r;
        k_exponent.mul_assign(&self.get_tag(&poly, &x_c));
        challange_exponent.mul_assign(&self.get_tag(&poly, &Fr::zero()));

        let mut k_f = G1::one();
        k_f.mul_assign(k_exponent);

        let mut g_r = G1::one();
        g_r.mul_assign(r);

        let mut g_r_0 = G1::one();
        g_r_0.mul_assign(challange_exponent);

        (k_f, (g_r, x_c, g_r_0))
    }
}
