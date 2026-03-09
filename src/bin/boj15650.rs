use algorithm::io::{Reader, Writer};

fn f(v: &mut Vec<bool>, a: &mut Vec<usize>, m: usize, w: &mut Writer) {
    if m == 0 {
        w.writeln(
            a.iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" "),
        );
        return;
    }
    let st = a.last().unwrap_or(&0);
    for i in *st + 1..v.len() {
        if !v[i] {
            v[i] = true;
            a.push(i);
            f(v, a, m - 1, w);
            a.pop();
            v[i] = false;
        }
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let m = r.next::<usize>();

    let mut v = vec![false; n + 1];
    let mut a = Vec::with_capacity(m);

    f(&mut v, &mut a, m, &mut w);
}
