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

The `std` crate compiles successfully, but fails at the linker stage. Here's how
that can be tested:

1. Change the source code in `src/main.rs` to use the commented out `main`
   function and `#![restricted_std]`.

2. In the source code for Rust's `std` crate, change a `cfg_if` in
   `$HOME/.rustup/toolchains/<your-host-nightly-toolchain>/lib/rustlib/src/rust/library/backtrace/src/backtrace/mod.rs`
   to use the `noop` trace instead of depending on `libunwind`. 

```diff
--- mod.rs	2022-06-21 12:52:21.724053459 +0530
+++ mod.rs	2022-06-21 13:05:50.948777093 +0530
@@ -132,7 +132,7 @@
         pub(crate) mod miri;
         use self::miri::trace as trace_imp;
         pub(crate) use self::miri::Frame as FrameImp;
-    } else if #[cfg(
+    } /* else if #[cfg(
         any(
             all(
                 unix,
@@ -154,7 +154,7 @@
         pub(crate) use self::dbghelp::Frame as FrameImp;
         #[cfg(target_env = "msvc")] // only used in dbghelp symbolize
         pub(crate) use self::dbghelp::StackFrame;
-    } else {
+    } */ else {
         mod noop;
         use self::noop::trace as trace_imp;
         pub(crate) use self::noop::Frame as FrameImp;
```

I haven't figured out what config I should give to `cargo` so I don't need to do
this. I find it surprising that `std` cannot be built without relying on
`libunwind`.

3. The build command now changes to

```bash
cargo +nightly build -Zbuild-std=core,alloc,panic_abort,libc,std -Zbuild-std-features=  --target=./x86_64-unknown-linux-cosmo.json
```

4. At the linker stage you might find some of the following symbols are missing:
  
  - If `Unwind_Backtrace` or similar is missing, you need to check if step 2 is
    done properly.

  - `open64`, `stat64`, `fstat64`, `__xpg_strerror_r`: these can be added to
    Cosmopolitan Libc via aliases of the existing functions.

  - `pthread_key_create`,`pthread_setspecific`,`pthread_key_delete`,`pthread_getspecific`
    -- these functions should not be needed in a single-threaded program, but
    somehow they are still linked. Support for threads is currently being added
    to Cosmopolitan Libc, but possibly the related code in the `std` crate can
    be changed to avoid this issue.

[rust]: https://rust-lang.org
[cosmo]: https://github.com/jart/cosmopolitan
[amalg-download]: https://justine.lol/cosmopolitan/download.html
[custom-target]: https://doc.rust-lang.org/rustc/targets/custom.html
[custom-embed]: https://docs.rust-embedded.org/embedonomicon/custom-target.html
