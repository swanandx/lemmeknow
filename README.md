<h1 align="center">
    lemmeknow 
</h1>

<div align="center">
  ⚡ 🦀 🔍
</div>
<div align="center">
  <strong>Just ask `lemmeknow`!</strong>
</div>
<div align="center">
  The fastest way to identify anything
</div>

<br />

<div align="center">
  <!-- Twitter -->
  <a href="https://twitter.com/_swanandx">
    <img src="https://img.shields.io/badge/twitter-%40__swanandx-blue"
      alt="@_swanandx" />
  </a>
  <!-- GitHub issues -->
  <a href="https://github.com/swanandx/lemmeknow/issues">
    <img src="https://img.shields.io/github/issues/swanandx/lemmeknow"
      alt="GitHub issues" />
  </a>
  <!-- GitHub stars -->
  <a href="https://github.com/swanandx/lemmeknow/stargazers">
    <img src="https://img.shields.io/github/stars/swanandx/lemmeknow"
      alt="GitHub stars" />
  </a>
  <!-- GitHub forks -->
  <a href="https://github.com/swanandx/lemmeknow/network">
    <img src="https://img.shields.io/github/forks/swanandx/lemmeknow"
      alt="GitHub forks" />
  </a>
  <!-- GitHub license -->
  <a href="https://github.com/swanandx/lemmeknow/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/swanandx/lemmeknow"
      alt="GitHub license" />
  </a>
</div>

<div align="center">
  <sub>Built with ❤︎ by
  <a href="https://twitter.com/_swanandx">swanandx</a> and
  <a href="https://github.com/swanandx/lemmeknow/graphs/contributors">
    contributors
  </a>
</div>

<br />

<!-- Thnx to choo for above README design <3 https://github.com/choojs/choo/blob/master/README.md -->

`lemmeknow` can be used for identifying mysterious text or to analyze hard-coded strings from captured network packets, malwares, or just about anything.

<br />

## 🧰 Usage 

<br />

Pass `TEXT` or `/PATH/TO/FILE` as argument 

- `lemmeknow [OPTIONS] <TEXT/FILENAME>`
 

If you want output in JSON, then pass `--json`

- `lemmeknow UC11L3JDgDQMyH8iolKkVZ4w --json` 


Run `lemmeknow --help` for more usages!

![demo](images/demo.gif)


## 🔭 Installation

<br />

### Download executable 📈

 You can directly download executable and run it. No need for any installation.
 > Check releases [here](https://github.com/swanandx/lemmeknow/releases/).

<br />

### Using `cargo` 🦀

- `cargo install lemmeknow`

<br />

### Build it from source 🎯

Clone repository

- `git clone https://github.com/swanandx/lemmeknow && cd lemmeknow`

then build and run
- `cargo run`

e.g. `cargo run -- <TEXT/FILENAME> [OPTIONS]`

*OR*

- `cargo build --release`
- `cd target/release/`
- `./lemmeknow`

e.g. `./lemmeknow <TEXT/FILENAME> [OPTIONS]`

<br />

## 🚀 API  

Want to use this as a crate in your project? or make a web api for it? No worries! Just add a entry in your `Cargo.toml`

```toml
[dependencies]
lemmeknow = "0.2.0"

```

OR 

```toml
[dependencies]
lemmeknow = { git = "https://github.com/swanandx/lemmeknow" }

```

Refer to [documentation](https://docs.rs/lemmeknow) for more info.

<br />

## 🚧 Contributing 

You can contribute by adding new regex, improving current regex, improving code performance or fixing minor bugs! Just open a issue or submit a PR.

<br />

## 💖 Acknowledgement

 This project is inspired by [PyWhat](https://github.com/bee-san/pyWhat)!
 Thanks to developer of it for the awesome idea <3 .
