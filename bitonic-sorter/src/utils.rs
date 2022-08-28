use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    //RNGを初期化する。再現性を持たせるため毎回同じシード値を使う
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sort_ascending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sort_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}
