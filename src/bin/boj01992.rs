use algorithm::io::{Reader, Writer};

fn solve(img: &Vec<Vec<char>>, i: usize, j: usize, n: usize) -> String {
    let first = img[i][j];
    if img[i..i + n]
        .iter()
        .all(|row| row[j..j + n].iter().all(|&x| x == first))
    {
        format!("{}", first)
    } else {
        let p1 = solve(img, i, j, n >> 1);
        let p2 = solve(img, i, j + (n >> 1), n >> 1);
        let p3 = solve(img, i + (n >> 1), j, n >> 1);
        let p4 = solve(img, i + (n >> 1), j + (n >> 1), n >> 1);
        format!("({}{}{}{})", p1, p2, p3, p4)
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let img = (0..n)
        .map(|_| r.next::<String>().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    w.write(solve(&img, 0, 0, n));
}
