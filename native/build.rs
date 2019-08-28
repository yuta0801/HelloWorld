extern crate neon_build;
extern crate cc;

fn main() {
    neon_build::setup(); // must be called in build.rs

    // add project-specific build logic here...
    cc::Build::new()
        .cpp(true)
        .file("src/lib.cpp")
        .compile("lib");
}
