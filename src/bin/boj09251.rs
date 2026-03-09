use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let word1 = r.next::<String>().chars().collect::<Vec<_>>();
    let word2 = r.next::<String>().chars().collect::<Vec<_>>();

    let n = word1.len();
    let m = word2.len();
    let mut dp = vec![0usize; (n + 1) * (m + 1)];

    for i in 1..=n {
        for j in 1..=m {
            if word1[i - 1] == word2[j - 1] {
                dp[i * (m + 1) + j] = dp[(i - 1) * (m + 1) + (j - 1)] + 1;
            } else {
                dp[i * (m + 1) + j] = dp[(i - 1) * (m + 1) + j].max(dp[i * (m + 1) + (j - 1)]);
            }
        }
    }

    w.write(dp[n * m + m + n]);
}
