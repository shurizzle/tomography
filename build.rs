fn main() {
    if std::env::var("TARGET").unwrap().contains("-apple") {
        println!("cargo:rustc-link-search=framework=/System/Library/PrivateFrameworks");
    }
}
