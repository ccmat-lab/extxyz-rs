use bindgen::builder;

fn main() {
    // compile extxyz
    let mut build = cc::Build::new();
    build
        .include("./extxyz/libcleri/inc/")
        .include("./extxyz/libextxyz/")
        .include("./grammar-gen/")
        .file("./extxyz/libextxyz/extxyz.c")
        .file("./grammar-gen/extxyz_kv_grammar.c")
        .compile("extxyz");

    // compile libcleri
    // let build = cc::Build::new();
    // build.file("./extxyz/libcleri/src/")
    //

    // Configure and generate bindings.
    let bindings = builder()
        .header("./wrapper/extxyz_wrapper.h")
        .header("./grammar-gen/extxyz_kv_grammar.h")
        .clang_arg("-I./extxyz/libcleri/inc/")
        .blocklist_type("FILE")
        .raw_line("use libc::FILE;")
        .allowlist_function("compile_extxyz_kv_grammar")
        .allowlist_function("extxyz_.*")
        .generate()
        .expect("unable to generate bindings");

    bindings
        .write_to_file("./src/bindings.rs")
        .expect("Couldn't write bindings!");
}
