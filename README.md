<div align="center">

# whitespace-sifter

![crates.io version](https://img.shields.io/crates/v/whitespace-sifter.svg?label=release)
![github.com forks](https://img.shields.io/github/forks/JumperBot/whitespace-sifter)
![github.com stars](https://img.shields.io/github/stars/JumperBot/whitespace-sifter)
![crates.io downloads](https://img.shields.io/crates/d/whitespace-sifter.svg?label=downloads)

</div>

---

```rust
use whitespace_sifter::*;

// This prints `1.. 2.. 3.. 4.. 5..`.
println!(
    "{}",
    sift("1.. \n2..  \n\r\n\n3..   \n\n\n4..    \n\n\r\n\n\n5..     \n\n\n\n\n")
);

// This prints `1..\n2..\n3..\n4..\r\n5..`.
println!(
    "{}",
    sift_preserve_newlines(
        "1.. \n2..  \n\r\n3..   \n\n\n4..    \r\n\n\r\n\n5..     \n\n\n\n\n"
    )
);
```

---

## ✨ Sift Duplicate Whitespaces In One Function Call

This crate **helps you** remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within the `&str`.  
Other than that, it naturally removes the whitespaces at the start and end of the `&str` using [`str::trim()`](https://doc.rust-lang.org/std/primitive.str.html#method.trim).

---

## ⚡️Benchmarks

Performance is one of the priorities of this crate.  
One of the advises is to not listen to repository authors/maintainers when it comes to benchmarks.  
You are free to run `cargo bench` on your machine after cloning this repository instead.  
These are the `String`s used to benchmark this crate:

```rust
format!(
     "{}\n\n{}\n\n{}\n\r\n\n{}\r\n\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
     "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
     "With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
     "Whitespaces.",
     "This\u{0020}\u{0020}is\u{0020}\u{0020}\u{0020}a\u{0020}\u{0020}sentence...",
     "With\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}\u{0020}some\u{0020}\u{0020}duplicate...",
     "Whitespaces."
)
// And
format!(
    "{}\n\n{}\n\n{}\n\n\n{}\r\n\n\r\n{}\r\n\r\n{}\r\n\r\n\r\n",
    "This. \n\nis. \n\na. \n\nsentence... \n\n",
    "With. \n\nsome. \n\nduplicate... \n\n",
    "Whitespaces. \n\n",
    "This. \r\n\r\nis. \r\n\r\na. \r\n\r\nsentence... \r\n\r\n",
    "With. \r\n\r\nsome. \r\n\r\nduplicate... \r\n\r\n",
    "Whitespaces. \r\n\r\n"
)
```

Execute these commands to benchmark:

```bash
$ git clone https://github.com/JumperBot/whitespace-sifter.git
$ cd whitespace-sifter
$ cargo bench
```

You should only look for 2 out of 4 benchmarks that look like the following:

```bash
Sift preserved/Loop Sift
                        time:   [844.28 ns 844.75 ns 845.21 ns]
                        change: [-2.9161% -1.0275% +0.3801%] (p = 0.29 > 0.05)
                        No change in performance detected.
Found 13 outliers among 100 measurements (13.00%)
  6 (6.00%) low mild
  5 (5.00%) high mild
  2 (2.00%) high severe

Sift/Loop Sift          time:   [645.18 ns 646.98 ns 649.86 ns]
                        change: [-0.7310% -0.1503% +0.4542%] (p = 0.67 > 0.05)
                        No change in performance detected.
Found 20 outliers among 100 measurements (20.00%)
  8 (8.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild
  7 (7.00%) high severe
```

---

## 📄 Licensing

`whitespace-sifter` is licensed under the [`MIT LICENSE`](./LICENSE); This is the [`summarization`](https://choosealicense.com/licenses/mit/).
