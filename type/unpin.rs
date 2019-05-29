use std::mem;
use std::pin::Pin;

fn main() {
  let mut string = "foo".to_string();
  let mut pinned_string = Pin::new(&mut string);

  // `String` implements `Unpin` that implicitly ensures a mutable reference to
  // `mem::replace` by invoking `Pin::deref_mut`.
  mem::replace(&mut *pinned_string, "bar".to_string());

  println!("{}", pinned_string);
}
