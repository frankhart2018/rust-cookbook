use rand::Rng;

fn main() {
    // Integers are uniformly distributed over the range of the type
    // Floating point numbers are uniformly distributed from 0 up to but not including 1
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random f64: {}", rng.gen::<f64>());
}
