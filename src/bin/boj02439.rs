use algorithm::io::{Reader, Writer};
fn main(){
    let mut r = Reader::new();
    let mut w = Writer::new();

    let n = r.next::<usize>();
    let mut stars = String::with_capacity(2 * n);
    for i in 0.. (n << 1){
        if i >= n {stars.push('*');}
        else {stars.push(' ');}
    }

    for i in 0..n{
        w.writeln(stars[i + 1..i + n + 1].to_string());
    }
}