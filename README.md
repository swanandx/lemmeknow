# For comparing performace

This branch is only for comparsion between performace of onig vs regex crate!

We uses regex crate by default, if you want to use onig crate, turn on `onig` feature.

## steps to compile binaries

```bash
# first compile binary which uses regex crate and copy it somewhere
cargo build --release
cp ./target/release/lemmeknow ./lmk_re

# Now compile for onig and copy
cargo build --release --features onig
cp ./target/release/lemmeknow ./lmk_onig
```

## For cargo bench

To benchmark the API, we use criterion.

```bash
cargo bench #regex crate
cargo bench --features onig #onig crate
```

