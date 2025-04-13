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

This crate **helps you** remove duplicate [whitespaces](https://doc.rust-lang.org/reference/whitespace.html) within a UTF-8 encoded `string`.  
It naturally removes the whitespaces at the start and end of the `string`.

---

## ⚡️Benchmarks

Performance is a priority; Most updates are performance improvements.  
The benchmark uses a transcript of the [Bee Movie](https://movies.fandom.com/wiki/Bee_Movie/Transcript).

Execute these commands to benchmark:

```bash
$ git clone https://github.com/JumperBot/whitespace-sifter.git
$ cd whitespace-sifter
$ cargo bench
```

You should only look for results that look like the following:

```bash
Sift/Sift               time:   [178.69 µs 178.84 µs 179.03 µs]
Sift Preserved/Sift Preserved
                        time:   [179.61 µs 179.75 µs 179.90 µs]
```

In just 0.0001 seconds; Pretty impressive, no?

<details>
<summary>Go try it on a better machine, I guess.</summary>
Benchmark specifications:  
<ul>
<li>Processor: Intel(R) Core(TM) i5-8350U CPU @ 1.70GHz 1.90 GHz</li>
<li>Memory: RAM 16.0 GB (15.8 GB usable)</li>
<li>System: GNU/Linux 5.15.153.1-microsoft-standard-WSL2 x86_64</li>
<li>Modified: v2.3.4</li>
</ul>
</details>

---

## 📈 Crate Comparison

Using the same benchmark configuration:

| Crate                                                                         | Dictionary          | Features          |  Time   | Complete Sift |
| ----------------------------------------------------------------------------- | ------------------- | ----------------- | :-----: | :-----------: |
| [whitespace-sifter](https://crates.io/crates/whitespace-sifter)               | ASCII Whitespaces   | Preserve Newlines | ~170 µs |      ✅       |
| [collapse](https://crates.io/crates/collapse)                                 | Unicode Whitespaces |                   | ~270 µs |      ✅       |
| [fast_whitespace_collapse](https://crates.io/crates/fast_whitespace_collapse) | ASCII Space and Tab | SIMD              | ~160 µs |      ❌       |

##### Disclaimers:

###### 1: I do not know the crate maintainers nor asked for permission to include their crates here.

###### 2: As far as I know, there are only three crates dedicated to whitespace sifting/collapse.

###### 3: `fast_whitespace_collapse` was not able to collapse cr-lf and line feeds.

| Dictionary                                                                                         | Characters                                          |
| -------------------------------------------------------------------------------------------------- | --------------------------------------------------- |
| [ASCII Whitespaces](https://doc.rust-lang.org/core/primitive.char.html#method.is_ascii_whitespace) | `'\t' \| '\n' \| '\x0C' \| '\r' \| ' '`             |
| [Unicode Whitespaces](https://doc.rust-lang.org/core/primitive.char.html#method.is_whitespace)     | `' ' \| '\x09'..='\x0d' \| unicode::White_Space(c)` |
| ASCII Space and Tab                                                                                | `' ' \| '\t'`                                       |

---

## 🔊 Changelog

- Improved Performance
- Minimum Supported Rust Version set to `v1.79.0` (starting `v2.3.3`)
- Stricter Tests (starting `v2.3.2`)
  - Proper UTF-8/Unicode Encoding
  - Regular Sifting
  - Sifting With Leading Whitespaces
  - Documentation Assertion
  - MSRV Verification
- Crate Comparison (starting `v2.3.4`)

---

## 📄 Licensing

`whitespace-sifter` is licensed under the [`MIT LICENSE`](./LICENSE); This is the [`summarization`](https://choosealicense.com/licenses/mit/).
