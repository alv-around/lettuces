use lettuces::ff::KyberParams;
use lettuces::ml_kem::{MlKem, PubKeyEncryption};
use rand::{SeedableRng, rngs::SmallRng};

fn main() {
    const N: usize = 32;
    let mut rng = SmallRng::from_seed([0u8; 32]);
    let msg = "private message";

    let key = <MlKem<N, KyberParams> as PubKeyEncryption>::keygen(&mut rng);
    let cypher = <MlKem<N, KyberParams> as PubKeyEncryption>::encrypt(msg, &key);
    <MlKem<N, KyberParams> as PubKeyEncryption>::decrypt(&cypher, &key);
}
