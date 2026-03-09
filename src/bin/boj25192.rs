use algorithm::io::{Reader, Writer};
use std::collections::HashSet;

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut cnt = 0;
    let mut set = HashSet::<String>::new();
    for _ in 0..n {
        match r.next::<String>().as_str() {
            "ENTER" => {
                set.clear();
            }
            name => {
                if !set.contains(name) {
                    set.insert(name.to_string());
                    cnt += 1;
                }
            }
        }
    }
    w.write(cnt);
}
