
fn main() {
    println!("cargo:rustc-link-search=native={}", "lua51");
    //TODO
    //println!("cargo:rustc-link-lib=dylib=luajit-sys/src/lua51");
}
