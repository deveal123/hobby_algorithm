use algorithm::io::{Reader, Writer};

fn push(v: &mut Vec<usize>, num: usize) {
    if v.len() == 0 {
        v.push(num);
        return;
    }

    if v[0] >= num {
        v[0] = num;
        return;
    }

    if v[v.len() - 1] < num {
        v.push(num);
        return;
    }

    let mut left = 0;
    let mut right = v.len() - 1;
    while left + 1 < right {
        let mid = (left + right) >> 1;
        if v[mid] < num {
            left = mid;
        } else {
            right = mid;
        }
    }
    v[right] = num;
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let numbers = (0..n).map(|_| r.next::<usize>()).collect::<Vec<_>>();

    let mut lis = Vec::with_capacity(n);
    for num in numbers {
        push(&mut lis, num);
    }
    w.write(lis.len());
}
