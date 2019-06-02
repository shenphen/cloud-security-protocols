use super::super::utils::{eval_poly_at_point, PolynomialRepr};
use super::types::{Challange, File, IdFile, TaggedFile};
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
        let polynomial_repr = self.get_polynomial_repr(id_file);
        let mut tag_block: TaggedFile = vec![];

        for m in file {
            tag_block.push((m, eval_poly_at_point(&polynomial_repr, &m)));
        }

        tag_block
    }

    fn get_polynomial_repr(&self, id_file: IdFile) -> PolynomialRepr {
        let mut poly: PolynomialRepr = vec![];
        let sk = self.sk.into_repr().0;
        let seed: &[usize] = &[sk[0] as usize, id_file];
        let mut rng: StdRng = SeedableRng::from_seed(seed);

        for i in 0..self.z {
            let a_i = Fr::rand(&mut rng);
            poly.push((a_i, i));
        }

        poly
    }

    pub fn gen_challange(&self, id_file: IdFile) -> (G1, Challange) {
        let polynomial_repr = self.get_polynomial_repr(id_file);
        let mut rng = rand::thread_rng();
        let r = Fr::rand(&mut rng);
        let x_c = Fr::rand(&mut rng);
        let mut k_exponent = r;
        let mut challange_exponent = r;
        k_exponent.mul_assign(&eval_poly_at_point(&polynomial_repr, &x_c));
        challange_exponent.mul_assign(&eval_poly_at_point(&polynomial_repr, &Fr::zero()));

        let mut k_f = G1::one();
        k_f.mul_assign(k_exponent);

        let mut g_r = G1::one();
        g_r.mul_assign(r);

        let mut g_r_0 = G1::one();
        g_r_0.mul_assign(challange_exponent);

        (k_f, (g_r, x_c, g_r_0))
    }
}
