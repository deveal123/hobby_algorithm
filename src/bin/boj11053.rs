use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());

    let n = r.next::<usize>();
    let a = (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>();

    let mut dp = vec![0usize; 1001];
    for num in a {
        dp[num] = (0..num).map(|i| dp[i]).max().unwrap() + 1;
    }
    w.write(dp.iter().max().unwrap());
}
