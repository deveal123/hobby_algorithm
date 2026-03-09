use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, k, x) = (r.next::<usize>(), r.next::<usize>(), r.next::<usize>());

    let power = (0..n)
        .map(|_| (r.next::<usize>(), r.next::<usize>()))
        .collect::<Vec<(usize, usize)>>();

    let mut dp = vec![false; (k + 1) * (x * k + 1)];
    dp[0] = true;

    for a in 1..=n {
        for b in 1..=a.min(k) {
            for c in 0..=(x * b) {
                let ind = b * (x * k + 1) + c;
                let mut flag = false;
                if (a - 1) >= b {
                    flag = flag || dp[b * (x * k + 1) + c];
                }
                if c >= power[a - 1].0 {
                    flag = flag || dp[(b - 1) * (x * k + 1) + c - power[a - 1].0];
                }
                dp[ind] = flag;
            }
        }
    }

    let st = k * (x * k + 1);
    let res = &dp[st..st + x * k + 1];

    let score = res
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v { Some(i * (x * k - i)) } else { None })
        .max();
    w.writeln(score.unwrap_or(0));
}
