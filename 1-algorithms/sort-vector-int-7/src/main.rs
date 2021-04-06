fn main() {
    let mut vec = vec![1, 5, 10, 2, 15];

    vec.sort();

    // Alternatively vec::sort_unstable could also be used
    // This is faster but does not preserve order of equal elements

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
