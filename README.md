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

## Remarks

- When used as CLI tool, onig outperformed regex.

![image](https://user-images.githubusercontent.com/73115739/193270158-2ad26a30-2e7b-4eac-baa9-9e3078bf99b8.png)

- Regex also generated lots of false positives, e.g.

![image](https://user-images.githubusercontent.com/73115739/193270269-069ac12f-5aa5-4328-81b4-ef76b6469075.png)

- But when we use criterion to benchmark it as a API by using `cargo bench`, regex performs better than onig!
