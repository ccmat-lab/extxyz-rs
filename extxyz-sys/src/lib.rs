#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unnecessary_transmutes)]

include!("bindings.rs");

fn foo() {
    unsafe {
        let kv_grammar = compile_extxyz_kv_grammar();
    }
}
