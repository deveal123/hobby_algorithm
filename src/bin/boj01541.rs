use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let s = r.next::<String>();

    let mut current_num = 0;
    let mut minus = 1;

    let mut res = 0;

    for c in s.chars() {
        if c == '-' {
            res += (minus * current_num);
            minus = -1;
            current_num = 0;
        } else if c == '+' {
            res += (minus * current_num);
            current_num = 0;
        } else {
            current_num = current_num * 10 + (c as u32 - '0' as u32) as i32;
        }
    }
    res += (minus * current_num);
    w.write(res);
}
