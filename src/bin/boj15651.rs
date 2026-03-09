use algorithm::io::{Reader, Writer};

fn f(n: usize, a: &mut Vec<usize>, m: usize, w: &mut Writer) {
    if m == 0 {
        w.writeln(
            a.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        );
        return;
    }
    for i in 1..n + 1 {
        a.push(i);
        f(n, a, m - 1, w);
        a.pop();
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let m = r.next::<usize>();

    let mut a = Vec::with_capacity(m);

    f(n, &mut a, m, &mut w);
}
