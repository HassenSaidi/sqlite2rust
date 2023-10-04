# sqlite2rust

This document describes the use of c2rust to translate the popular sqlite database to rust. Rust supports bindings for the sqlite database through the sqlite-binding crate. But there are no re-implementations of sqlite in rust. The document describes the process of using c2rust to translate the C implementation into a running rust implementation of sqlite. c2rust does produce rust code that needs patching and fixing to be able to build. This document describes the good, bad, and ugly parts of using c2rust as a general process for migrating a significant C code base into rust, and proposes a new framework for a C2RUST built as a translation assistant.

## Using c2rust

