use std::collections::BTreeMap;

use algorithm::io::{Reader, Writer};

fn main(){
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, m) = (r.next::<usize>(), r.next::<usize>());

    let mut count = BTreeMap::<String, usize>::new();

    (0..(n + m)).for_each(|_|{
        let name = r.next::<String>();
        *count.entry(name).or_insert(0) += 1;
    });
    
    let result = count.iter().filter(|(_, v)| **v == 2).map(|(k, _)| k).collect::<Vec<&String>>();
    w.writeln(result.len());
    result.iter().for_each(|&k| w.writeln(k));
}