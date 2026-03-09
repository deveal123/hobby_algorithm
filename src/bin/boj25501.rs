use algorithm::{
    io::{Reader, Writer},
    string::StringIndexTrait,
};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    for _ in (0..n) {
        let word = r.next::<String>();
        let rev_word = word.chars().rev().collect::<String>();
        let l = word.len();
        let mut cnt = 1;
        let mut is_palindrome = true;
        for i in (0..l >> 1) {
            if word.char_at(i) == rev_word.char_at(i) {
                cnt += 1;
            } else {
                is_palindrome = false;
                break;
            }
        }

        w.writeln(format!("{} {}", if is_palindrome { 1 } else { 0 }, cnt));
    }
}
