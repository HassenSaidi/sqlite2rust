# sqlite2rust

This document describes the use of [c2rust](https://github.com/immunant/c2rust) to translate the popular [SQLite](https://sqlite.org/) database to [rust](https://www.rust-lang.org/). SQLite is a very popular database installed on billions of devices. Rust supports bindings for the SQLite database through the [SQLite crate](https://crates.io/crates/sqlite). But there are no re-implementations of SQLite in Rust. The document describes the process of using c2rust to translate the C implementation into a running Rust implementation of SQLite. c2rust does produce Rust code that needs patching and fixing to be able to build. This document describes the good, bad, and ugly parts of using c2rust as a general process for translating a significant C code base into Rust.

## Using c2rust
Here are the steps for translating SQLite C source code into Rust:

### Download c2rust

The easiest way to download c2rust is to use the following commad which points to a specific version of `llvm-config`.

```bash
LLVM_CONFIG_PATH=/usr/bin/llvm-config-14 cargo install c2rust
```
### Download compiledb

`compiledb` is a tool for intercepting the build process of the target C application. The result is a `compile_commands.json` that can be used by c2rust as follows:

```bash
c2rust transpile path/to/compile_commands.json
```

To download `compiledb`, run the following command
```bash
pip install compiledb
```

### Download SQLite
Download the source code [here](https://sqlite.org/download.html). The current version is https://sqlite.org/2023/sqlite-autoconf-3430100.tar.gz.
The source code for SQLite is organized in three main files:

* a header file `sqlite3.h`.
* a library file `sqlite3.c` that corresponds to the output library `libsqlite3.a`.
* a shell wrapper for running SQLite from the commandline `shell.c` that corresponds to the output executable `sqlite3`.

### Generate compile_commands.json

in the SQLite source directory, run the folliwng:

```bash
# After running
./autogen.sh && ./configure # etc.
# Run
compiledb make
```

This will produce in the same directory a `compile_commands.json` file. This describes how the `make` builds `libsqlite3.a` and `sqlite3`.

### Running c2rust

in the same directory, running the following commands translate the C code into Rust code

```bash
c2rust transpile --emit-build-files compile_commands.json
```

The result of the build is two Rust files `sqlite3.rs` and `shell.rs`.

### Building the Rust project

To build the output files into a Rust executable, we use the following `Cargo.toml` file that describes the build of an executable called `sqlite_in_rust`. This file is the Rust binary corresponding to the `sqlite3` C binary. We create an `src` directory where we move `shell.c` to `src/main.rs` and `sqlite3.rs` to `src/lib.rs`.
We use a build file `build.rs` to pass locally installed system libraries dependencies such as `zlib` and `readline`.

```bash
# Cargo.toml

[package]
name = "sqlite_in_rust"
version = "0.1.0"
edition = "2021"
build = "build.rs"

[lib]
path = "src/lib.rs"


[[bin]]
name = "sqlite3_in_rust"
path = "src/main.rs"


[dependencies]
libc = "0.2"
f128 = "0.2.9"
num-traits = "0.2.16"
c2rust-bitfields = "0.18.0"

```

The `build.rs` files contains the following

```rust
fn main() {
    println!("cargo:rustc-flags=-lz -lreadline");
}
```

### Patching the Rust code to build the Rust project

## Evaluation

## stats

Here are the stats for the original C code
| Original C File Name | Lines of Code | Binary size |
| -------------------- | ------------- | ----------- |
| sqlite3.c            | 250808        |             |
| shell.c              | 28615         | `sqlite3` 7962544     |

Here are the stats for the generated Rust code
| Original Rust File Name | Lines of Code | Binary size |
| ----------------------- | ------------- | ----------- |
| sqlite3.rs              | 212608        |             |
| shell.rs                | 41717         | `sqlite3_in_rust` 19893720    |

Here are the stats for building the Rust project:

| Errors | Warnings |
| ------ | -------- |
| 7      | 6294     |

After fixing the errors, 6294 warnings remain. running `cargo  +nightly  fix --bin "sqlite3_in_rust"` reduces the number of warnings to 4282. There are two types of warnings:
1. warning: `variable/function fOo should have a snake case name` with the siggestion that it should be rewritten as `f_oo`.
2. warning: `path statement with no effect`.


### The good

After fixing the Rust output and getting rid of the errors, we get a working binary that seems to be working fine. Success!

```bash
vagrant@vagrant:/vagrant/sqlite-autoconf-3430100$ ./target/debug/sqlite3_in_rust 
SQLite version 3.43.1 2023-09-11 12:01:27
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
sqlite>
```

The tool c2rust seems to be working!
A significant code base such as SQLite can be made to work with little effort. 
The tool produces unsafe Rust code that closely mirrors the input C code. T
he primary goal of c2rust is to produce code that is functionally identical to the input C code.
A good understanding or C and Rust is necessary to fix the Rust build errors
and ignore warnings that are not important to the correctness of the output.
SQLite code structure is simple as it exists in two source files and a header file.
More complicated build of C projects require a bit more effort to build a Rust 
project out of the generate Rust files. But overwall, the effort can be managed. 


### The bad

The current state of c2rust produces Rust code that is functionally equivalent to the original C code. 
FOr instance, the produced code does not use appropriate Rust idioms. A refactoring tool is in the works to translate 
the unsafe Rust to an idiomatic safe Rust code. This might end up being a tall order as Rust is a richer 
language, and eliminating `unsafe` mightg require some advanced code analysis.

### The ugly

A maintainer of a significant C code base might not recognize their application in the generated Rust code.
A simple main function in C will turn into a Rust main function that calls the real main function 
called main_0 with a structure that differs from the original C main. 

Oddly, strings present in the original C code are not found in the Rust code. A deeper look into the inner working of c2rust might 
explain that. But the point is that the resulting Rust code will be hard to make sense of.

The resulting code is unmaintainable. It is hard to imagine a C developer using the translater 
(transpiler) to generated Rust code intended to be maintained and further developed. A complete
re-write of the code might be necessary. Not sure how a refactoring tool can render the resulting Rust 
code readable and maintainable.





