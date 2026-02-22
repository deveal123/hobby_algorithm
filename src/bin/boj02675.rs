use algorithm::io::{Reader, Writer};
use algorithm::string::*;
fn main(){
    let mut r = Reader::new();
    let mut w = Writer::new();

    let mut tc = r.next::<usize>();
    while tc != 0{
        let rep = r.next::<usize>();
        let word = r.next::<String>();

        let mut res = String::with_capacity(rep * word.len());

        for ch in word.iter(){
            (0..rep).for_each(|_|{res.push(ch);})
        }
        tc -= 1;
        w.writeln(res);
    }
}