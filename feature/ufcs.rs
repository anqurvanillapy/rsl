/**
 *  UFCS
 *  ====
 *
 *  Uniform (universal) function call syntax: Good for "dot-autocomplete", good
 *  for health.
 */

trait Foo {
    fn foo(&self);
    fn bar(&self);
}

trait Bar {
    fn bar(&self);
}

struct A;

impl Foo for A {
    fn foo(&self) {
        println!("Foo::foo");
    }

    fn bar(&self) {
        println!("Foo::bar");
    }
}

impl Bar for A {
    fn bar(&self) {
        println!("Bar::bar");
    }
}

fn main() {
    let a = A;

    // Kinda like OO vs procedural.
    a.foo();
    Foo::foo(&a);

    // The following fails in compilation due to ambiguity.
    // a.bar();
    // Disambiguate it using UFCS.
    Foo::bar(&a);
    Bar::bar(&a);
}
