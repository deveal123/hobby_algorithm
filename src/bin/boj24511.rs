use algorithm::io::{Reader, Writer};
use std::collections::VecDeque;

struct QueueStack<T: Copy> {
    ds: VecDeque<T>,
}

impl<T: Copy> QueueStack<T> {
    fn from(ty: Vec<bool>, val: Vec<T>) -> Self {
        let ds = ty
            .iter()
            .zip(val.iter())
            .filter_map(|(t, v)| if *t { Some(*v) } else { None })
            .collect();
        Self { ds }
    }

    fn pushpop(&mut self, val: T) -> T {
        self.ds.push_front(val);
        self.ds.pop_back().unwrap()
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let ty = (0..n).map(|_| r.next::<i32>() == 0).collect();
    let val = (0..n).map(|_| r.next::<i32>()).collect();
    let mut qs = QueueStack::from(ty, val);
    let m = r.next::<usize>();
    let values = (0..m).map(|_| r.next::<i32>()).collect::<Vec<_>>();
    for i in values {
        w.write(qs.pushpop(i));
    }
}
