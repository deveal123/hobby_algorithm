use algorithm::io::{Reader, Writer};
fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();

    let mut max_ind = 0usize;
    let mut max_num = 0;
    (0..9).map(|i| (i, r.next::<i32>())).for_each(|(ind, x)| {
        if x > max_num {
            max_num = x;
            max_ind = ind;
        }
    });
    w.write(format!("{}\n{}", max_num, max_ind + 1));
}
