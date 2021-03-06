extern crate rayon;

use rayon::prelude::*;

fn main() {
    // If there are multiple elements satisfying the predicate defined
    // in the closure argument of rayon::find_any, rayon returns
    // the first one found, not necessarily the first one

    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
}
                