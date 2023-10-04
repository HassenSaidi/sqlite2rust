# sqlite2rust

This document describes the use of [c2rust](https://github.com/immunant/c2rust) to translate the popular [sqlite](https://sqlite.org/) database to [rust](https://www.rust-lang.org/). Rust supports bindings for the sqlite database through the [SQLite crate](https://crates.io/crates/sqlite). But there are no re-implementations of sqlite in rust. The document describes the process of using c2rust to translate the C implementation into a running rust implementation of sqlite. c2rust does produce rust code that needs patching and fixing to be able to build. This document describes the good, bad, and ugly parts of using c2rust as a general process for translating a significant C code base into rust.

## Using c2rust
Here are the steps for translating sqlite C source code into rust:

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

### Download sqlite
Download the source code [here](https://sqlite.org/download.html). The current version is https://sqlite.org/2023/sqlite-autoconf-3430100.tar.gz.
The source code for sqlite is organized in three main files:

* a header file `sqlite3.h`.
* a library file `sqlite3.c` that corresponds to the output library `libsqlite3.a`.
* a shell wrapper for running sqlite from the commandline `shell.c` that corresponds to the output executable `sqlite3`.

### Generate compile_commands.json

in the sqlite source directory, run the folliwng:

```bash
# After running
./autogen.sh && ./configure # etc.
# Run
compiledb make
```

This will produce in the same directory a `compile_commands.json` file. This describes how the `make` builds `libsqlite3.a` and `sqlite3`.

### Running c2rust

in the same directory, running the following commands translate the C code into rust code

```bash
c2rust transpile --emit-build-files compile_commands.json
```

The result of the build is two rust files `sqlite3.rs` and `shell.rs`.

### Building the rust project

To build the output files into a rust executable, we use the following `Cargo.toml` file that describes the build of an executable called `sqlite_in_rust`. This file is the rust binary corresponding to the `sqlite3` C binary. We create an `src` directory where we move `shell.c` to `src/main.rs` and `sqlite3.rs` to `src/lib.rs`.
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

### Patching the rust code to build the rust project

## Evaluation

## stats

Here are the stats for the original C code
| Original C File Name | Lines of Code | Binary size |
| -------------------- | ------------- | ----------- |
| sqlite3.c            | 250808        |             |
| shell.c              | 28615         | `sqlite3` 7962544     |

Here are the stats for the generated rust code
| Original rust File Name | Lines of Code | Binary size |
| ----------------------- | ------------- | ----------- |
| sqlite3.rs              | 212608        |             |
| shell.rs                | 41717         | `sqlite3_in_rust` 19893720    |

Here are the stats for building the rust project:

| Errors | Warnings |
| ------ | -------- |
| 7      | 6294     |

After fixing the errors, 6294 warnings remain. running `cargo  +nightly  fix --bin "sqlite3_in_rust"` reduces the number of warnings to 4282. There are two types of warnings:
1. warning: `variable/function fOo should have a snake case name` with the siggestion that it should be rewritten as `f_oo`.
2. warning: `path statement with no effect`.


### The good

After fixing the rust output and getting rid of the errors, we get a working binary that seems to be working fine. Success!

```bash
vagrant@vagrant:/vagrant/sqlite-autoconf-3430100$ ./target/debug/sqlite3_in_rust 
SQLite version 3.43.1 2023-09-11 12:01:27
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
sqlite>
```

```c
"The tool c2rust seems to be working. A significant code base such as sqlite can be made to work with little effort. A good understanding or C and rust is necessary to fix the rust build errors and ignore warnings that are not important to the correctness of the output".
```
### The bad

```c
"The tool c2rust seems to be working. A significant code base such as sqlite can be made to work with little effort. A good understanding or C and rust is necessary to fix the rust build errors and ignore warnings that are not important to the correctness of the output.
```

### The ugly

```c
"The tool c2rust seems to be working. A significant code base such as sqlite can be made to work with little effort. A good understanding or C and rust is necessary to fix the rust build errors and ignore warnings that are not important to the correctness of the output.
```


