use algorithm::io::{Reader, Writer};
fn main(){
    let mut r = Reader::new();
    let mut w = Writer::new();

    let n = r.next::<usize>();
    let scores = (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>();
    let max_score = scores.iter().max().unwrap();
    let sum = scores.iter().sum::<usize>() * 100;
    w.write(sum as f64 / (max_score * n) as f64);
}