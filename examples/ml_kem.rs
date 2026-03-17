use lettuces::{
    ff::KyberParams,
    linalg::{Matrix, Vector},
};
use rand::{SeedableRng, rngs::SmallRng};

fn main() {
    let mut rng = SmallRng::from_seed([0u8; 32]);
    let _A: Matrix<8, KyberParams> = Matrix::random(&mut rng);

    println!("generating key for Alice");
    let _s: Vector<8, KyberParams> = Vector::random(&mut rng);

    println!("generating key for Bob");
    let _r: Vector<8, KyberParams> = Vector::random(&mut rng);
}
