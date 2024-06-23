# unsafe-libopus

This library is [libopus](https://github.com/xiph/opus) 1.3.1 translated from C to unsafe Rust with the assistance
of [c2rust](https://github.com/immunant/c2rust).

It is called "unsafe" libopus, because it still pretty much has the same shape as the C code and is full of "unsafe".

This translation still allows you to get rid of C compiler toolchain and use the library in a pure rust environment,
without linker hackery & dynamic linking issues.

It also may potentially be a starting point for a more idiomatic rust implementation of libopus, though I am not sure if
it is worth the effort.

## Usage

You can leverate this library by using a fork of `opus` crate
from [this PR](https://github.com/SpaceManiac/opus-rs/pull/20):

```toml
[dependencies]
opus = { git = "https://github.com/DCNick3/opus-rs.git", branch = "unsafe-libopus", default-features = false, features = ["unsafe-libopus-backend"] }
```

Maybe, this library will have the safe APIs in the future, but for now, it is a (mostly) drop-in replacement for
the `audiopus_sys` crate.

## Translation technique

Firstly, the [libopus 1.3.1](https://archive.mozilla.org/pub/opus/opus-1.3.1.tar.gz) was compiled with the following
commands:

```bash
CC=clang ./configure --disable-shared --disable-stack-protector --enable-extra-programs --disable-doc --disable-asm --disable-rtcd --disable-intrinsics --disable-dependency-tracking--disable-maintainer-mode --enable-hardening
CC=clang compiledb make -j
```

Then, using the resulting `compile_commands.json` file, the C code was transpiled to rust with:

```bash
c2rust transpile compile_commands.json -o . --overwrite-existing --reorganize-definitions --emit-modules --translate-const-macros --emit-build-files
```

The resulting code was then manually reorganized, to remove all duplication of structures & eradicate the use
of `#[no_mangle] extern "C"` functions: they are all linked with rust now and do not have to exported.

Some other refactorings include:

- separation of the test binaries from the library as "examples"
- removal of dependencies of libc functions in the library by defining their variants in the library (idea
  from [unsafe-libyaml](https://github.com/dtolnay/unsafe-libyaml))
- removal of dependency on the libc crate by replacing the libc types with concrete rust types (`libc::c_int` -> `i32`,
  etc)
- removal of C varargs by replacing them with a custom VarArgs struct, as C variadics are not yet stable in rust. This
  turns the `opus_*_ctl` family of functions into macros

## Performance

The library was translated without the use of inline assembly, processor intrinsics and runtime CPU detection, so it is
not as fast as the original code right now. The C version with those features is about 20% faster than the rust version
on my machine.

## Correctness

This library is tested using (most of) the original tests from the C codebase. They are present in form of rust
integration tests in the `tests` directory. Still not translated are a bunch of unit tests from the C codebase.

The crux of the testing happens in the `src/bin/run_vectors2.rs` though.
It tests both the decoder and encoder using IETF-published test vectors,
comparing the results with the C implementation of opus 1.3.1 located in `upstream-libopus`.
It does have a small number of patches to make the results more portable.

The decoder is tested by decoding the test vectors and comparing the results with the C implementation at a
number of output sample rates.
They are checked to match exactly.

The same is done to the encoder,
running at a number of different bitrates and comparing the encoded results with the C implementation.
Aside from bitrate, no other parameters are changed, which currently is a weak point of the testing.

Strictly speaking, same encoded results are not required for this to be a valid implementation: the encoder is free to
do a lot of choices, leaving to different quality results.
However, making a codec that is _better_ than the original opus is a non-goal. Therefore, by requiring the exact same
results, we prevent any divergence in the behavior of the encoder.

## Safety

Currently, most of the code is unsafe, as it is a direct transpilation of the C code.

I am currently in the process of slowly refactoring the code to make certain parts of it safe. Previously maintaining
validity was a challenge, but now that there are tests checking the implementation against the C code, it should become
easier.

## License

Same as the original libopus, `unsafe-libopus` is licensed under the BSD 3-clause license.
