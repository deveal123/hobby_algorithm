use algorithm::io::{Reader, Writer};


fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();
    let mut tc = r.next::<usize>();
    while tc != 0{
        let mut cost = r.next::<usize>();
        let mut res = [0usize;4];
        res[0] = cost / 25; cost = cost % 25;
        res[1] = cost / 10; cost = cost % 10;
        res[2] = cost / 5; cost = cost % 5;
        res[3] = cost;
        w.writeln(format!("{} {} {} {}", res[0], res[1], res[2], res[3]));
        tc -= 1;
    }
}