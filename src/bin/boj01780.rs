use algorithm::io::{Reader, Writer};

fn solve(paper: &Vec<Vec<i32>>, i: usize, j: usize, n: usize) -> (usize, usize, usize) {
    let first = paper[i][j];
    if paper[i..i + n]
        .iter()
        .all(|row| row[j..j + n].iter().all(|&x| x == first))
    {
        if first == -1 {
            (1, 0, 0)
        } else if first == 0 {
            (0, 1, 0)
        } else {
            (0, 0, 1)
        }
    } else {
        let (mut s1, mut s2, mut s3) = (0, 0, 0);
        (0..9).for_each(|idx| {
            let (a, b) = (idx / 3, idx % 3);
            let (ns1, ns2, ns3) = solve(paper, i + a * (n / 3), j + b * (n / 3), n / 3);
            s1 += ns1;
            s2 += ns2;
            s3 += ns3;
        });
        (s1, s2, s3)
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let paper = (0..n)
        .map(|_| (0..n).map(|_| r.next::<i32>()).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let (a, b, c) = solve(&paper, 0, 0, n);
    w.write(format!("{a}\n{b}\n{c}"));
}
