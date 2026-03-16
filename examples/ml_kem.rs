use lettuces::{ff::KyberParams, ml_kem::MlKem};
use rand::{SeedableRng, rngs::SmallRng};

fn main() {
    println!("generating key");
    let mut rng = SmallRng::from_seed([0u8; 32]);
    let ml_kem_key = MlKem::<8, KyberParams>::keygen(&mut rng);
    println!("generated key: {:?}", ml_kem_key);

    println!("MISSING: encrypt key");
    println!("MISSING: decrypt key");
}
