use libloading::{Library, Symbol};
use std::env;
use std::path::Path;

fn load_dynarmic() {
    unsafe {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("dynarmic/build/src/dynarmic/libdynarmic.6.5.dylib");
        println!("path: {:?}", path);
        let lib = Library::new(path).unwrap(); // Load the "hello_world" library
        let create_environment: Symbol<fn()> = lib.get(b"create_environment").unwrap(); // Load the "create_environment" symbol
        let dynarmic = create_environment();
        println!("dynarmic: {:?}", dynarmic);
    }
}

#[test]
fn test_load_dynarmic() {
    load_dynarmic();
}