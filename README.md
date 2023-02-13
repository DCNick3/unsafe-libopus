# unsafe-libopus

This library is [libopus](https://github.com/xiph/opus) 1.3.1 translated from C to unsafe Rust with the assistance of [c2rust](https://github.com/immunant/c2rust).

It is called "unsafe" libopus, because it still pretty much has the same shape as the C code and is full of "unsafe".

This translation still allows you to get rid of C compiler toolchain and use the library in a pure rust environment, without linker hackery & dynamic linking issues.

It also may potentially be a starting point for a more idiomatic rust implementation of libopus, though I am not sure if it is worth the effort.

## Usage

To use this library use the fork of opus crate or smth [TODO].

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
