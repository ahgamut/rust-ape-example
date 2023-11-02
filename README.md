# Cosmopolitan Libc and Rust

> Note: the executables built using Cosmo and Rust do not work across the
> operating systems supported by Cosmopolitan Libc, because Rust decides system
> constants like `EINVAL` [at
> compile-time](https://github.com/search?q=repo%3Arust-lang%2Flibc%20EINVAL&type=code)
> (see https://github.com/ahgamut/rust-ape-example/issues/3), based on the
> operating system provided in the compilation target. Thus, I expect the fat
> binaries built here will only work on `x86-64-linux` and `aarch64-linux`. Very
> well, perhaps in the future we can find a way to have system constants as
> `extern` values in Rust, like how [I did it for
> C](https://github.com/ahgamut/gcc/tree/portcosmo-11.2).

This repository contains a simple `Hello world!` example in the [Rust][rust]
programming language, that builds with [Cosmopolitan Libc][cosmo]. Now it also
includes all the example snippets I could scrape from [Rust By Example][rbe],
and it builds around 175 example programs, including those that use Rust's
`std::thread` and `std::sync::Arc`.

> `ripgrep` builds with Cosmopolitan Libc -- check it out
> [here](https://github.com/ahgamut/ripgrep/tree/cosmopolitan).


To build this repo you need a recent version of the Cosmopolitan Libc monorepo,
and `bash` because I wrote a simple filter script.

I created a [custom compilation target][custom-target] for Rust, called
`x86_64-unknown-linux-cosmo`, to provide a build process that uses the
Cosmopolitan Libc amalgamation and `cargo`. I followed the documentation in the
[Rust Embedonomicon][custom-embed] to create the target.

An alternative method to build APEs with Rust would be to avoid `cargo`, just
use `rustc` or equivalent compiler to generate `.o` files, and then write a
shell script that does the linking with the expected flags. I have not tried
this method.

## Building a Rust APE with the `std` crate

1. Download the Cosmopolitan Libc repo and build the toolchain:

```bash
git clone https://github.com/jart/cosmopolitan
cd cosmopolitan
make -j MODE= toolchain
make -j MODE=aarch64 toolchain
export COSMO=$(realpath ./)
cd ..
mkdir cosmos
export COSMOS=$(realpath ./cosmos)
$COSMO/bin/cosmocc --update
```

Then clone this repo

```bash
git clone https://github.com/ahgamut/rust-ape-example
cd rust-ape-example
```

2. Download the necessary host toolchain and source code for Rust:

```bash
# I was on Debian 11, so I did this
rustup toolchain install nightly-x86_64-unknown-linux-gnu
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
# on Alpine Linux, you may need to do
rustup toolchain install nightly-x86_64-unknown-linux-musl
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-musl
```

For reference, this worked when I tried it for `nightly-x86_64-linux-gnu` and:

* the Rust binaries on October 10 2023

3. run `cargo build` to get the debug executables. This uses a bash script that
   removes unnecessary linker arguments.

```bash
export ARCH=x86_64
cargo +nightly build --target=./x86_64-unknown-linux-cosmo.json
export ARCH=aarch64
cargo +nightly build --target=./aarch64-unknown-linux-cosmo.json
```


4. run `apelink` on the debug binaries to obtain the fat APE:

```bash
# look at the built debug binaries
ls ./target/x86_64-unknown-linux-cosmo/debug/*.com.dbg
ls ./target/aarch64-unknown-linux-cosmo/debug/*.com.dbg

# apelink
MODE=
MODE_AARCH64=aarch64
APELINK=$COSMO/o/tool/build/apelink.com
apelinkpls () {
    OUTPUT="$1"
    OUTPUT_X86_64="$2"
    OUTPUT_AARCH64="$3"
    "$APELINK" -l "$COSMO/o/$MODE/ape/ape.elf" \
        -l "$COSMO/o/$MODE_AARCH64/ape/ape.elf" \
        -M "$COSMO/ape/ape-m1.c" \
        -o "$OUTPUT" \
        "$OUTPUT_X86_64" \
        "$OUTPUT_AARCH64"
}

apelinkpls ./hello.com\
    ./target/x86_64-unknown-linux-cosmo/debug/hello.com.dbg\
    ./target/aarch64-unknown-linux-cosmo/debug/hello.com.dbg

# run the APE
./hello.com
```

Now we have Actually Portable Executables built with Rust! I also built a few
more executables using the code from [Rust By Example][rbe], and an APE that
doesn't use the `std` crate. There might some edge cases that I haven't noticed,
so clone/fork the repo and try it out!

## TODOs

- [ ] figure out build config to avoid using `libunwind`

The `std` crate relies on
[`backtrace`](https://github.com/rust-lang/backtrace-rs), which depends on
[`libunwind`](https://github.com/libunwind/libunwind) in the default builds for
unix. To work around this, `cosmopolitan.a` currently has stubs for the
functions that `backtrace` relies on. However, it might be easier to provide a
build flag in `Cargo.toml` to use the `noop` module of `backtrace`. 

A small change needs to be submitted to the source code of `backtrace` (in the
`cfg_if!`
[here](https://github.com/rust-lang/backtrace-rs/blob/4e5a3f72929f152752d5659e95bb15c8f6b41eff/src/backtrace/mod.rs#L128))
to allow choosing `noop` when building as part of the `std` crate. This
conditional compilation flag should be accessible when building the `std` crate
either via `Cargo.toml` or something like `-Z use-std-backtrace-noop` in the
build command.

[without-std-branch]: https://github.com/ahgamut/rust-ape-example/tree/without-std
[rust]: https://rust-lang.org
[rbe]: https://doc.rust-lang.org/rust-by-example/
[cosmo]: https://github.com/jart/cosmopolitan
[cosmo-nightly]: https://github.com/jart/cosmopolitan/commit/b69f3d2488dbaf9dcc541e699f5b7c09fbf046e0
[amalg-download]: https://justine.lol/cosmopolitan/download.html
[custom-target]: https://doc.rust-lang.org/rustc/targets/custom.html
[custom-embed]: https://docs.rust-embedded.org/embedonomicon/custom-target.html
