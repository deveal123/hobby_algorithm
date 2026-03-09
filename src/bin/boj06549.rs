use algorithm::io::{Reader, Writer};

fn solve(histogram: &[usize]) -> usize {
    if histogram.len() == 1 {
        return histogram[0];
    }
    let n = histogram.len() as i32;
    let (m1, m2) = (
        solve(&histogram[..(n as usize) >> 1]),
        solve(&histogram[(n as usize) >> 1..]),
    );
    let mut stk = Vec::with_capacity(n as usize);

    let (mut st, mut fi) = ((n >> 1) - 1, (n >> 1));
    let mut height = usize::MAX;

    while st >= 0 || fi < n {
        if st < 0 {
            height = histogram[fi as usize];
            while fi < n && histogram[fi as usize] >= height {
                fi += 1;
            }
            stk.push((height, st, fi));
        } else if fi >= n {
            height = histogram[st as usize];
            while st >= 0 && histogram[st as usize] >= height {
                st -= 1;
            }
            stk.push((height, st, fi));
        } else if histogram[st as usize] > histogram[fi as usize] {
            height = histogram[st as usize];
            while st >= 0 && histogram[st as usize] >= height {
                st -= 1;
            }
            stk.push((height, st, fi));
        } else {
            height = histogram[fi as usize];
            while fi < n && histogram[fi as usize] >= height {
                fi += 1;
            }
            stk.push((height, st, fi));
        }
    }
    let m3 = stk
        .iter()
        .map(|(h, s, f)| h * (f - s - 1) as usize)
        .max()
        .unwrap();
    m1.max(m2).max(m3)
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    loop {
        let n = r.next::<usize>();
        if n == 0 {
            break;
        }
        let histogram = (0..n).map(|_| r.next::<usize>()).collect::<Vec<usize>>();
        w.writeln(solve(&histogram[..]));
    }
}
