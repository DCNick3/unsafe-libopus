# unsafe-libopus

This library is [libopus](https://github.com/xiph/opus) 1.3.1 translated from C to unsafe Rust with the assistance of [c2rust](https://github.com/immunant/c2rust).

It is called "unsafe" libopus, because it still pretty much has the same shape as the C code and is full of "unsafe".

This translation still allows you to get rid of C compiler toolchain and use the library in a pure rust environment, without linker hackery & dynamic linking issues.

It also may potentially be a starting point for a more idiomatic rust implementation of libopus, though I am not sure if it is worth the effort.

## Usage

You can leverate this library by using a fork of `opus` crate from [this PR](https://github.com/SpaceManiac/opus-rs/pull/20):

```toml
[dependencies]
opus = { git = "https://github.com/DCNick3/opus-rs.git", branch = "unsafe-libopus", default-features = false, features = ["unsafe-libopus-backend"] }
```

Maybe, this library will have the safe APIs in the future, but for now, it is a (mostly) drop-in replacement for the `audiopus_sys` crate. 

## Translation technique

Firstly, the [libopus 1.3.1](https://archive.mozilla.org/pub/opus/opus-1.3.1.tar.gz) was compiled with the following commands:
```bash
CC=clang ./configure --disable-shared --disable-stack-protector --enable-extra-programs --disable-doc --disable-asm --disable-rtcd --disable-intrinsics --disable-dependency-tracking--disable-maintainer-mode --enable-hardening
CC=clang compiledb make -j
```

Then, using the resulting `compile_commands.json` file, the C code was transpiled to rust with:
```bash
c2rust transpile compile_commands.json -o . --overwrite-existing --reorganize-definitions --emit-modules --translate-const-macros --emit-build-files
```

The resulting code was then manually reorganized, to remove all duplication of structures & eradicate the use of `#[no_mangle] extern "C"` functions: they are all linked with rust now and do not have to exported.

Some other refactorings include:
- separation of the test binaries from the library as "examples"
- removal of dependencies of libc functions in the library by defining their variants in the library (idea from [unsafe-libyaml](https://github.com/dtolnay/unsafe-libyaml))
- removal of dependency on the libc crate by replacing the libc types with concrete rust types (`libc::c_int` -> `i32`, etc)
- removal of C varargs by replacing them with a custom VarArgs struct, as C variadics are not yet stable in rust. This turns the `opus_*_ctl` family of functions into macros  

## Performance

The library was translated without the use of inline assembly, processor intrinsics and runtime CPU detection, so it is not as fast as the original code right now. The C version with those features is about 20% faster than the rust version on my machine.

## Correctness & Safety

This library is tested using (most of) the original tests from the C codebase. They are present in form of rust integration tests in the `tests` directory.

The opus [test vectors](https://opus-codec.org/testvectors/) are also used to test the library decoder in CI (see the `src/bin/run_vectors.rs`). This pretty much ensures that the decoder is correct. This actually helped uncover a [subtle bug in c2rust](https://github.com/immunant/c2rust/issues/853).

The correctness of the encoder is a bit more of an open question, as there are no test vectors for it, AFAIK. There is some testing going on in `tests/opus_encode`, but it's quite limited.

As for the safety, no guarantees are made. The code is a result of a pretty direct transpilation of the C code, so it is almost entirely consists of unsafe code (in rust terms).

It also seems to have UB according to [miri](https://github.com/rust-lang/miri), but those are probably false positives. A good refactoring goal might be to make the library pass miri checks. 
