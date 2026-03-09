use algorithm::io::{Reader, Writer};

fn solve(paper: &Vec<Vec<usize>>, i: usize, j: usize, n: usize) -> (usize, usize) {
    let first = paper[i][j];
    if paper[i..i + n]
        .iter()
        .all(|row| row[j..j + n].iter().all(|&x| x == first))
    {
        if first == 1 { (1, 0) } else { (0, 1) }
    } else {
        let p1 = solve(paper, i, j, n >> 1);
        let p2 = solve(paper, i, j + (n >> 1), n >> 1);
        let p3 = solve(paper, i + (n >> 1), j, n >> 1);
        let p4 = solve(paper, i + (n >> 1), j + (n >> 1), n >> 1);
        (p1.0 + p2.0 + p3.0 + p4.0, p1.1 + p2.1 + p3.1 + p4.1)
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let paper = (0..n)
        .map(|_| (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let (blue, white) = solve(&paper, 0, 0, n);
    w.write(format!("{}\n{}", white, blue));
}
