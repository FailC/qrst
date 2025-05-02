
### Very simple CLI-tool for generating QR-codes

__Create and print QR-code to the terminal__
```bash
qrst -i github.com/failc/qrst
```

__Save as png (with optional filename) and custom resolution__
```bash
qrst -i github.com --png github.png -r 800
```

__Installation from source:__

```bash
git clone https://github.com/FailC/qrst.git
cd qrst
cargo install --path .
```
if you only want to run it:
```
cargo run -- -i github.com
```
(basically just clap + fast_qr https://crates.io/crates/fast_qr)
i wrote this 80 line program to avoid having to use some qr-code generation website and for an excuse to write more Rust. Did i mention that this very important project is written in Rust??
Why even read this?
