use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    // Randomluy generates a string of given length (30) ASCII characters
    // in range A-Z, a-z, 0-9 with Alphanumeric sample
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("{}", rand_string);
}
