use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, m) = (r.next::<usize>(), r.next::<usize>());

    let rem = (0..n)
        .scan(0, |acc, _| {
            *acc = (*acc + r.next::<usize>()) % m;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    let mut count = vec![0; m];
    count[0] = 1;
    rem.iter().for_each(|a| {
        count[*a] += 1;
    });

    w.write(
        count
            .iter()
            .map(|&x| if x > 0 { x * (x - 1) / 2 } else { 0 })
            .sum::<usize>(),
    )
}
