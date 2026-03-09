use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let mut v = Vec::<String>::new();
    v.push("-".to_string());
    for _ in (0..12) {
        let mut s = String::new();
        let ss = v.last().unwrap();
        s.push_str(ss);
        s.push_str(&" ".repeat(ss.len()));
        s.push_str(ss);
        v.push(s);
    }
    loop {
        match r.try_next::<usize>() {
            Ok(n) => {
                w.writeln(v[n].clone());
            }
            Err(_) => break,
        }
    }
}
