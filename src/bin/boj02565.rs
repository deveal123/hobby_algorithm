use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut a = (0..n)
        .map(|_| (r.next::<usize>(), r.next::<usize>()))
        .collect::<Vec<_>>();
    a.sort();
    let locate = a.iter().map(|x| x.1).collect::<Vec<_>>();

    let mut dp = vec![0usize; 501];
    for num in locate {
        dp[num] = dp[0..num].iter().max().unwrap() + 1;
    }
    w.write(n - dp.iter().max().unwrap());
}
