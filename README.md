<p align='center'>
<img src='images/logo.gif'>
<p align="center">
<i>The fastest way to identify anything</i><br>
</p>

# lemmeknow :zap:

Identify any mysterious text or analyze strings from a file, just ask `lemmeknow`.

`lemmeknow` can be used for identifying mysterious text or to analyze hard-coded strings from captured network packets, malwares, or just about anything, for identifying

- All URLs
- Emails
- Phone numbers
- Credit card numbers
- Cryptocurrency addresses
- Social Security Numbers
- and much more.


# :toolbox: Usage

If you have executable, the just pass TEXT or /PATH/TO/FILE as argument e.g. `lemmeknow secrets.pcap` and it will determine if the argument is a file or just a text and then perform analysis accordingly!

If you want output in JSON, then pass `--json`, e.g. `lemmeknow UC11L3JDgDQMyH8iolKkVZ4w --json` 

![demo](images/demo.gif)


# :telescope: Installation


### Download executable :chart_with_upwards_trend:

 You can directly download executable and run it. No need for any installation.
 - Check releases [here](https://github.com/swanandx/lemmeknow/releases/).


### Using `cargo` :crab:

- `cargo install lemmeknow`


### Build it from source :dart:

Clone repository

- `git clone https://github.com/swanandx/lemmeknow && cd lemmeknow`

then build and run
- `cargo run`
e.g. `cargo run -- <TEXT/FILENAME> [OPTIONS]`

OR

- `cargo build --release`
- `cd target/release/`
- `./lemmeknow`
e.g. `./lemmeknow <TEXT/FILENAME> [OPTIONS]`


# :scream_cat: API 

Want to use this as a crate in your project? or make a web api for it? No worries! Just add a entry in your Cargo.toml

```toml
[dependencies]
lemmeknow = "0.1.0"

```

OR 

```toml
[dependencies]
lemmeknow = { git = "https://github.com/swanandx/lemmeknow" }

```

Refer to [documentation](https://docs.rs/lemmeknow) for more info.

# :construction: Contributing

You can contribute by adding new regex, improving current regex, improving code performance or fixing minor bugs! Just open a issue or submit a PR.

# Acknowledgement
 This project is inspired by [PyWhat](https://github.com/bee-san/pyWhat)!
 Thanks to developer of it for the awesome idea <3 .
