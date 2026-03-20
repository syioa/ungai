![Banner](assets/logo.png)

# ungai

![Rust](https://img.shields.io/badge/Language-Rust-orange)
![License](https://img.shields.io/badge/License-MIT-blue)

> A high-performance unique name generator based on Markov Chains

## Installation

You will need a [Rust Toolchain](https://rust-lang.org/tools/install/) installed on your system.

Then run

```bash
cargo install ungai
```

This will install ungai binary on your system.

## Usage

Run

```bash
ungai --help
```
or
```bash
ungai -h
```

The `help` message is well documented all thanks to the amazing [clap](https://github.com/clap-rs/clap) crate.


## FAQs

### What is the difference between `smoothing` and `temperature`?

`smoothing` adds a certain value to every transition fixing dataset gaps.

On the other hand `temperature` controls the randomness of the data and increases output variety.

>[!NOTE]
> Both affect the creativity of the model.

