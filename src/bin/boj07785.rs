use std::collections::HashSet;

use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();

    let mut residuals = HashSet::<String>::new();
    (0..n).for_each(|_| {
        let name = r.next::<String>();
        match r.next::<String>().as_str() {
            "enter" => {
                residuals.insert(name);
            }
            "leave" => {
                residuals.remove(&name);
            }
            _ => {}
        }
    });

    let mut v = residuals.into_iter().collect::<Vec<_>>();
    v.sort_unstable_by_key(|a| a.clone());
    v.reverse();
    v.into_iter().for_each(|name| {
        w.writeln(name);
    });
}
