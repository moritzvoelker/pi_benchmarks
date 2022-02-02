// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/hello.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/pi.c")
        .flag("-O3")
        .compile("pi_c");
    nasm_rs::Build::new()
        .file("src/pi.asm")
        .flag("-f elf64")
        .compile("pi_asm")
        .unwrap();
}