fn main() {
    println!("cargo:rerun-if-changed=user.ld");
    println!("cargo:rustc-link-arg=-Tuser.ld");
    println!("cargo::rustc-link-arg=-Map=user.map");
}
