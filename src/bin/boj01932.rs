use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut cost = vec![0usize; n * n];
    for i in 1..=n {
        cost[(i - 1) * n..(i - 1) * n + i]
            .iter_mut()
            .for_each(|x| *x = r.next::<usize>());
    }

    for i in 2..=n {
        cost[(i - 1) * n] += cost[(i - 2) * n];
        for j in 1..i {
            cost[(i - 1) * n + j] += cost[(i - 2) * n + (j - 1)].max(cost[(i - 2) * n + j]);
        }
    }

    w.writeln(cost[n * n - n..n * n].iter().max().unwrap());
}
