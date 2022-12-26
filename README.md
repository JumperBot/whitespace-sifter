# whitespace-sifter

Sift duplicate whitespaces away in just one function call.

This crate helps you remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str`.  
Other than that, it naturally removes the whitespaces at the start and end of the `&str`.

# Examples

```rust-lang
use whitespace_sifter::*;

fn main() {
  // This prints `1.. 2.. 3.. 4.. 5..`.
  println!("{}", sift("1.. \n2..  \n\n3..   \n\n\n4..    \n\n\n\n5..     \n\n\n\n\n"));

  // This prints `A..\r\nB..\r\nC..\r\nD..\r\nE..`.
  println!("{}", sift_with_carriage_return("A..\r\n B..\r\n\r\n C..\r\n\r\n\r\n D..\r\n\r\n\r\n\r\n E..\r\n\r\n\r\n\r\n\r\n"));
}
```
