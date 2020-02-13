/// Type `Void` has no value, a.k.a. the *false proposition*.
enum Void {}

/// In Rust there is a *never* type to simulate the absurd pattern.
// fn false_prop() -> ! {
//     unimplemented!()
// }

/// But I prefer using the `Void`.
fn false_prop() -> Void {
    unimplemented!()
}

fn main() {
    false_prop();
}
