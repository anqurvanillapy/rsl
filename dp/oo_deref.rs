use std::ops::Deref;

/**
 *  `Deref`-based OO
 *  ================
 *
 *  An infamous anti-pattern: The deref polymorphism.
 */

struct Base {}

impl Base {
    fn foo(&self) {
        println!("base");
    }
}

struct Derived {
    b: Base,
}

impl Deref for Derived {
    // The resulting type after dereferencing.
    type Target = Base;

    fn deref(&self) -> &Base {
        &self.b
    }
}

fn main() {
    let d = Derived { b: Base {} };
    d.foo();
}
