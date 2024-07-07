#[no_mangle]
pub fn main() {
    let s = "Hello, userland!\n";
    for c in s.bytes() {
        crate::util::putchar(c);
    }
    loop {}
}
