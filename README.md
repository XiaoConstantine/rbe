Mini bpe in Rust
----------------
[![Rust](https://github.com/XiaoConstantine/rbe/actions/workflows/rust.yml/badge.svg)](https://github.com/XiaoConstantine/rbe/actions/workflows/rust.yml)


Port [minbpe](https://github.com/karpathy/minbpe) to rust as learning process

Benchmark
---------
**Build binary**

```bash
cargo build --release

```

**Run tokenizer**

```bash
./target/release/rbpe --tokenizer {basic, regex}
```

**Results**

On my m1 book, I got:

| Mode | Time (rust)  |Time (python) |
|---------|-------|-|
| Basic   | 0.4s  |5.65s |
| Regex   | 1.23s |9.01s |
