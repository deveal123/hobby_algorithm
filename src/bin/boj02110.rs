use algorithm::io::{Reader, Writer};

fn solve(arr: &Vec<usize>, c: usize, n: usize) -> bool {
    let mut curr = 0;
    let mut curr_num = arr[0];

    for num in arr {
        if num - curr_num >= c {
            curr += 1;
            curr_num = *num;
        }
    }

    curr >= n
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, c) = (r.next::<usize>(), r.next::<usize>() - 1);
    let mut x = (0..n).map(|_| r.next::<usize>()).collect::<Vec<usize>>();
    x.sort();

    let (mut l, mut r) = (1, x[n - 1] - x[0] + 1);

    while l + 1 < r {
        let mid = (l + r) / 2;
        if solve(&x, mid, c) {
            l = mid;
        } else {
            r = mid;
        }
    }

    w.write(l);
}
