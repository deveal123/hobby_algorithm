use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();

    let mut v = vec![vec![' '; n]; n];

    fn f(v: &mut Vec<Vec<char>>, x: usize, y: usize, n: usize) {
        if n == 1 {
            v[x][y] = '*';
            return;
        }
        let m = n / 3;
        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }
                f(v, x + i * m, y + j * m, m);
            }
        }
    }
    f(&mut v, 0, 0, n);
    for i in (0..n) {
        let s: String = v[i].iter().collect();
        w.writeln(s);
    }
}
