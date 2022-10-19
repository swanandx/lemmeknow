<h1 align="center">
    lemmeknow 
</h1>

<div align="center">
  ⚡ 🦀 🔍
</div>
<div align="center">
  <strong>Just ask "lemmeknow"!</strong>
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


<div align="center">
  <strong>Try it online 
  <a href="https://swanandx.github.io/lemmeknow-frontend/">here</a>
  </strong>
</div>

<br />

<div align="center">
  <strong>Watch the video on 
  <a href="https://youtu.be/n92YrzELBJU">YouTube</a>
  </strong>
</div>

<br />


## 🧰 Usage

```shell
lemmeknow [OPTIONS] <TEXT/FILENAME>
```
<img align="center" src="https://media.discordapp.net/attachments/869896750188625950/1032250968043171881/unknown.png" alt="demo"/>

<details>
<summary>
JSON Output
</summary>

If you want output in JSON format, then pass `-j / --json` flag.
*e.g.* 
```shell
lemmeknow UC11L3JDgDQMyH8iolKkVZ4w --json
``` 
<img align="center" src="https://media.discordapp.net/attachments/998569651183288351/1009151747194892288/lkjosn.png?width=1440&height=512" alt="demo" />
</details>

> Run `lemmeknow --help` for all options!



<br />

## 🔭 Installation
---
### Download executable 📈

 You can directly download executable and run it. No need for any installation.
 > Check releases [here](https://github.com/swanandx/lemmeknow/releases/).

---
### Using `cargo` 🦀
```shell
cargo install lemmeknow
```

---
### Build it from source 🎯

Clone repository
```shell
git clone https://github.com/swanandx/lemmeknow && cd lemmeknow
```

then build and run

```shell
cargo run -- <TEXT/FILENAME> [OPTIONS]
```

OR

```shell
cargo build --release
cd target/release/
./lemmeknow <TEXT/FILENAME> [OPTIONS]
```

---
<br />

## 🚀 API  

Want to use this as a crate in your project? or make a web api for it? No worries! Just add a entry in your `Cargo.toml`

```toml
[dependencies]
lemmeknow = { version = "0.7", default-features = false }

```

OR 

```toml
[dependencies]
lemmeknow = { git = "https://github.com/swanandx/lemmeknow", default-features = false }

```

> Refer to [documentation](https://docs.rs/lemmeknow) for more info.

lemmeknow supports webassembly, that is, it can be compiled for `wasm32-unknown-unknown` target!
e.g. [lemmeknow-frontend](https://github.com/swanandx/lemmeknow-frontend)

<br />

## ⚔️ Benchmarks

lemmeknow is around **33x faster** than pywhat for a file of 8.7MB, and it is **3x faster** for a single string!

| A file of 8.7MB | A single string |
| ---  |  ---   |
| *Summary*: `lemmeknow.exe floss.exe` ran **33.13 ± 9.74** times faster than `pywhat floss.exe` | *Summary*: `lemmeknow.exe 3FZ..Zc5` ran **3.29 ± 0.77** times faster than `pywhat 3FZ..Zc5` |
| ![File benchmark](images/bench_file.png)     | ![String benchmark](images/bench_string.png)       |

> Thanks to [SkeletalDemise](https://github.com/SkeletalDemise) for the benchmarks and the whisker plots ✨

<br />

## 🚧 Contributing 

You can contribute by adding new regex, improving current regex, improving code performance or fixing minor bugs! Just open a issue or submit a PR.

<br />

## 💖 Acknowledgement

 This project is inspired by [PyWhat](https://github.com/bee-san/pyWhat)!
 Thanks to developer of it for the awesome idea <3 .
