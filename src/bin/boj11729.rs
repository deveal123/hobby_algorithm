use algorithm::io::{Reader, Writer};

fn hanoi(n: usize, src: usize, dst: usize, aux: usize, w: &mut Writer) {
    if n == 1 {
        w.writeln(format!("{} {}", src, dst));
        return;
    }
    hanoi(n - 1, src, aux, dst, w);
    w.writeln(format!("{} {}", src, dst));
    hanoi(n - 1, aux, dst, src, w);
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    w.writeln((1 << n) - 1);
    hanoi(n, 1, 3, 2, &mut w);
}
