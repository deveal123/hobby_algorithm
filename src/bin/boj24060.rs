use algorithm::io::{Reader, Writer};
fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, k) = (r.next::<usize>(), r.next::<usize>());
    let v = (0..n).map(|_| r.next::<i32>()).collect::<Vec<i32>>();
    let mut arr = vec![0; n + 1];

    if n > 2 {
        arr[2] = 2;
    }

    for i in (3..=n) {
        arr[i] = arr[i >> 1] + arr[i - (i >> 1)] + i;
    }

    fn find_kth(v: &[i32], k: usize, arr: &[usize]) -> Option<i32> {
        let n = v.len();
        if k > arr[n] {
            return None;
        }
        if n & 1 == 0 {
            if k <= arr[n >> 1] {
                return find_kth(&v[..n >> 1], k, arr);
            } else if k <= 2 * arr[n >> 1] {
                return find_kth(&v[n >> 1..], k - arr[n >> 1], arr);
            } else {
                let mut vv = v.to_vec();
                vv.sort();
                return Some(vv[k - 2 * arr[n >> 1] - 1]);
            }
        } else {
            if k <= arr[(n + 1) >> 1] {
                return find_kth(&v[..(n + 1) >> 1], k, arr);
            } else if k <= arr[(n + 1) >> 1] + arr[n >> 1] {
                return find_kth(&v[(n + 1) >> 1..], k - arr[(n + 1) >> 1], arr);
            } else {
                let mut vv = v.to_vec();
                vv.sort();
                return Some(vv[k - arr[(n + 1) >> 1] - arr[n >> 1] - 1]);
            }
        }
    }

    match find_kth(v.as_slice(), k, arr.as_slice()) {
        Some(x) => w.writeln(x),
        None => w.writeln(-1),
    }
}
