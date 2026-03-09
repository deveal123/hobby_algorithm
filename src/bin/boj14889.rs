use std::collections::HashSet;

use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let power = (0..(n * n)).map(|_| r.next::<i32>()).collect::<Vec<_>>();
    let mut mp = (0..((n >> 1) + 1))
        .map(|_| HashSet::<i32>::new())
        .collect::<Vec<_>>();

    mp[0].insert(0);

    let sums = (0..n)
        .map(|i| {
            let mut s = 0;
            for j in (0..n) {
                s += power[(n * i + j) as usize];
                s += power[(n * j + i) as usize];
            }
            s
        })
        .collect::<Vec<_>>();

    let offset: i32 = power.iter().sum();

    for s in sums {
        for i in (1..((n >> 1) + 1)).rev() {
            let new_items: Vec<i32> = mp[i - 1].iter().map(|&num| num + s).collect(); // It will be faster if we use bitset
            mp[i].extend(new_items);
        }
    }

    let m = mp[n >> 1]
        .iter()
        .map(|&num| (num - offset).abs())
        .min()
        .unwrap();
    w.write(m);
}
