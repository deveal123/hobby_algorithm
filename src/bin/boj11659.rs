use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let m = r.next::<usize>();

    let sum = (0..n)
        .scan(0, |acc, _| {
            *acc += r.next::<usize>();
            Some(*acc)
        })
        .collect::<Vec<_>>();

    (0..m).for_each(|_| {
        let st = match r.next::<usize>() {
            0 | 1 => 0,
            i => sum[i - 2],
        };
        let en = sum[r.next::<usize>() - 1];
        w.writeln(en - st);
    });
}
