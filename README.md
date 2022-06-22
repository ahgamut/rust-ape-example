# Actually Portable Executables with Cosmopolitan Libc and Rust

This repository contains a simple `Hello world!` example in the [Rust][rust]
programming language, that builds with [Cosmopolitan Libc][cosmo]. 

I created a [custom compilation target][custom-target] for Rust, called
`x86_64-unknown-linux-cosmo`, to provide a build process that uses the
Cosmopolitan Libc amalgamation and `cargo`. I followed the documentation in the
[Rust Embedonomicon][custom-embed] to create the target.

An alternative method to build APEs with Rust would be to avoid `cargo`, just
use `rustc` or equivalent compiler to generate `.o` files, and then write a
shell script that does the linking with the expected flags. I have not tried
this method.

## Build steps

1. Download the Cosmopolitan Libc [amalgamation][amalg-download] into the `libcosmo` folder:

```bash
cd libcosmo
wget https://justine.lol/cosmopolitan/cosmopolitan.zip
unzip cosmopolitan.zip
cd ../
```

2. Download the necessary *host* toolchain and source code for Rust:

```bash
# I was on Debian, so I did this
rustup toolchain install nightly-x86_64-unknown-linux-gnu
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
# on Alpine Linux, you may need to do
rustup toolchain install nightly-x86_64-unknown-linux-musl
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-musl
```

3. run `cargo build` to get the debug executable. This uses a bash script that
   removes unnecessary linker arguments. A recent version of `gcc` and `ld.bfd`
   is required.

```bash
cargo +nightly build -Zbuild-std=core,libc --target=./x86_64-unknown-linux-cosmo.json
```

4. run `objcopy` on the debug binary to obtain the APE:

```bash
objcopy -SO binary ./target/x86_64-unknown-linux-cosmo/debug/hello_world.com.dbg ./hello_world.com
```

## What about the `std` crate?

1. It needs a few stubs functions so the linker doesn't complain. I wrote them
   out in `stubs.c`.

```bash
cd ./libcosmo/
# compiling a simple stub file via cosmopolitan.h
./compile-stubs.bash
```

2. The build command now changes to

```bash
cargo +nightly build -Zbuild-std=core,alloc,panic_abort,libc,std -Zbuild-std-features=  --target=./x86_64-unknown-linux-cosmo.json
```

[rust]: https://rust-lang.org
[cosmo]: https://github.com/jart/cosmopolitan
[amalg-download]: https://justine.lol/cosmopolitan/download.html
[custom-target]: https://doc.rust-lang.org/rustc/targets/custom.html
[custom-embed]: https://docs.rust-embedded.org/embedonomicon/custom-target.html
