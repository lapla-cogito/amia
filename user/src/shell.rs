#[no_mangle]
pub fn main() {
    loop {
        let mut cnt = 0;
        let mut buf = [0u8; 512];
        crate::util::print("> ");

        loop {
            let ch = crate::util::getchar();
            crate::util::putchar(ch);
            if cnt > 511 {
                crate::util::print("too long input");
                break;
            } else if ch == b'\r' {
                break;
            } else {
                buf[cnt] = ch;
                cnt += 1;
                continue;
            }
        }

        if &buf[..cnt] == b"ping" {
            crate::util::print("pong\n");
        } else if &buf[..cnt] == b"exit" {
            crate::util::exit();
        } else {
            crate::util::print("unknown command\n");
        }
    }
}
