use algorithm::{
    io::{Reader, Writer},
    string::StringIndexTrait,
};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let word = r.next::<String>();

    let q = r.next::<usize>();
    let mut s = vec![0usize; 26 * word.len()];

    for i in (0..26 * word.len()) {
        if word.char_at(i / 26) as usize - 'a' as usize == i % 26 {
            s[i] += 1;
        }
        if i >= 26 {
            s[i] += s[i - 26];
        }
    }

    for _ in 0..q {
        let ch = r.next::<char>() as usize - 'a' as usize;
        let l = r.next::<usize>();
        let r = r.next::<usize>();
        w.writeln(s[26 * r + ch] - if l > 0 { s[26 * (l - 1) + ch] } else { 0 });
    }
}
