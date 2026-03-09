use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let word = r.next::<String>();
    let mut word = word.chars().collect::<Vec<char>>();
    word.sort();
    word.reverse();
    w.writeln(word.iter().collect::<String>());
}
