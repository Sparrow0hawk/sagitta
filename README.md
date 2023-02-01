# Sagitta

A Rust CLI tool for retrieving job information from SGE accounting files.

## Setup

To install this tool you'll need to install [Rust](https://www.rust-lang.org/). 
I'd recommend doing this with [rustup](https://rustup.rs/) which installs the full toolchain.

Once you've installed the Rust toolchain you can clone this repository and install it with `cargo`.

```bash
$ git clone https://github.com/Sparrow0hawk/sagitta.git

$ cd sagitta

$ cargo install --path .
```

That will install sagitta as a binary in `~/.cargo/bin`.

## Usage

You can use `sagitta` to return job information from the SGE accounting file by passing a job ID.

```bash
$ sagitta --help
Usage: sagitta [-j|--job-id=JOB_ID] FILE
```

```bash
$ sagitta -j 66 path/to/accounting/file
```
