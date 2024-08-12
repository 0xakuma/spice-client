fn main() {
    println!("cargo:rustc-link-lib=spice-client-glib-2.0");
    println!("cargo:rustc-link-lib=gio-2.0");
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
}
