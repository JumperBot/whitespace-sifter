<div align="center">

# whitespace-sifter

![crates.io version](https://img.shields.io/crates/v/whitespace-sifter.svg?label=release)
![github.com forks](https://img.shields.io/github/forks/JumperBot/whitespace-sifter)
![github.com stars](https://img.shields.io/github/stars/JumperBot/whitespace-sifter)
![crates.io downloads](https://img.shields.io/crates/d/whitespace-sifter.svg?label=downloads)

</div>

---

```rust
use whitespace_sifter::WhitespaceSifter;
// This prints `1.. 2.. 3.. 4.. 5..`.
println!(
    "{}",
    "1.. \n2..  \n\r\n\n3..   \n\n\n4..    \n\n\r\n\n\n5..     \n\n\n\n\n".sift(),
);

// This prints `1..\n2..\n3..\n4..\r\n5..`.
println!(
    "{}",
    "1.. \n2..  \n\r\n3..   \n\n\n4..    \r\n\n\r\n\n5..     \n\n\n\n\n"
        .sift_preserve_newlines(),
);
```

---

## ✨ Sift Duplicate Whitespaces In One Function Call

This crate **helps you** remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within a `string`.  
Other than that, it naturally removes the whitespaces at the start and end of the `string` using [`str::trim()`](https://doc.rust-lang.org/std/primitive.str.html#method.trim).

---

## ⚡️Benchmarks

Performance is one of the priorities of this crate.  
One of the advises is to not listen to repository authors/maintainers when it comes to benchmarks.  
You are free to run `cargo bench` on your machine after cloning this repository instead.  
The benchmark uses a transcript of the [Bee Movie](https://movies.fandom.com/wiki/Bee_Movie/Transcript).

Execute these commands to benchmark:

```bash
$ git clone https://github.com/JumperBot/whitespace-sifter.git
$ cd whitespace-sifter
$ cargo bench
```

You should only look for results that look like the following:

```bash
Sift/Sift               time:   [284.16 µs 310.00 µs 339.70 µs]
Sift Preserved/Sift Preserved
                        time:   [391.47 µs 400.57 µs 414.17 µs]
```

Not even half a second; Pretty impressive, no?  
Go try it on a better machine, I guess.

---

## 📄 Licensing

`whitespace-sifter` is licensed under the [`MIT LICENSE`](./LICENSE); This is the [`summarization`](https://choosealicense.com/licenses/mit/).
