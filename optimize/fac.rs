/**
 *  Factorial
 *  =========
 *
 *  Nothing's here, please leave.  Actually, rustc guarantees the compile-time
 *  evaluation in a certain level in -O.
 */

fn fac(n: i32) -> i64 {
    let mut term = 1;
    for i in 1..n + 1 {
        term *= i as i64;
    }
    term
}

fn main() {
    println!("{}", fac(10));
}
