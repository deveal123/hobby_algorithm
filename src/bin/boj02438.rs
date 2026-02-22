use algorithm::io::{Reader, Writer};
fn main(){
    let mut r = Reader::new();
    let mut w = Writer::new();
    let stars = ["*"; 100];

    for i in 0..r.next::<usize>(){
        w.writeln(format!("{}", stars[0..(i + 1)].join("")));
    }
}