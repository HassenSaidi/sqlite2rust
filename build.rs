// SQLite requires zlib and readline
// This build makes sure that the linker finds locally installed zlib and readline

fn main() {
    println!("cargo:rustc-flags=-lz -lreadline");
}
