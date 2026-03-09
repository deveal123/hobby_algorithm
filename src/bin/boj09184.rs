use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());

    let mut dp = [1; 21 * 21 * 21];

    for i in 463usize..9261usize {
        let (a, b, c) = (i / 441, (i % 441) / 21, i % 21);
        if a == 0 || b == 0 || c == 0 {
            continue;
        }
        if a < b && b < c {
            dp[i] = dp[i - 1] + dp[i - 22] - dp[i - 21];
        } else {
            dp[i] = dp[i - 441] + dp[i - 462] + dp[i - 442] - dp[i - 463];
        }
    }

    loop {
        let a = r.next::<i32>();
        let b = r.next::<i32>();
        let c = r.next::<i32>();
        if a == -1 && b == -1 && c == -1 {
            break;
        }
        if a <= 0 || b <= 0 || c <= 0 {
            w.writeln(format!("w({a}, {b}, {c}) = 1"));
        } else if a > 20 || b > 20 || c > 20 {
            w.writeln(format!("w({a}, {b}, {c}) = 1048576"));
        } else {
            w.writeln(format!(
                "w({a}, {b}, {c}) = {}",
                dp[a as usize * 441 + b as usize * 21 + c as usize]
            ));
        }
    }
}
