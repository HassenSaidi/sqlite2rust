# sqlite2rust

This document describes the use of [c2rust](https://github.com/immunant/c2rust) to translate the popular [sqlite](https://sqlite.org/) database to [rust](https://www.rust-lang.org/). Rust supports bindings for the sqlite database through the [SQLite crate](https://crates.io/crates/sqlite). But there are no re-implementations of sqlite in rust. The document describes the process of using c2rust to translate the C implementation into a running rust implementation of sqlite. c2rust does produce rust code that needs patching and fixing to be able to build. This document describes the good, bad, and ugly parts of using c2rust as a general process for translating a significant C code base into rust.

## Using c2rust
Here are the steps for translating sqlite C source code into rust:

### Download c2rust

The easiest way to download c2rust is to use the following commad:

```bash
cargo install c2rust
```

### Download sqlite
Download the source code [here](https://sqlite.org/download.html). The current version is https://sqlite.org/2023/sqlite-autoconf-3430100.tar.gz.

