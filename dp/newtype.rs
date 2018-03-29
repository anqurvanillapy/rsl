/**
 *  Newtype
 *  =======
 *
 *  Nicely create a more concrete and incompatible new type with an old type and
 *  its set of interfaces.
 */

#[derive(Default)]
struct Sum<T> {
    pub val: T,
}

impl<T> Sum<T> {
    fn add(&mut self, a: T, b: T) -> T
    where
        T: std::ops::Add<Output = T> + Clone,
    {
        self.val = a + b;
        self.val.clone()
    }
}

// The newtype.
#[derive(Default)]
struct Mul<T>(Sum<T>);

impl<T> Mul<T> {
    fn mul(&self, a: T, b: T) -> T
    where
        T: std::ops::Mul<Output = T>,
    {
        // There is no self.val in the newtype, even it's a `pub' member.
        // self.val = a * b;
        a * b
    }
}

fn main() {
    let mut s: Sum<i32> = Default::default();
    debug_assert!(s.add(0, 42) == 42);

    let m: Mul<i32> = Default::default();
    debug_assert!(m.mul(0, 42) == 0);

    // Sum<T> and Mul<T> are incompatible.
    // let s: Sum<i32> = m;
}
