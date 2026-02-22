use std::collections::HashMap;

use algorithm::io::{Reader, Writer};

fn main(){
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, m) = (r.next::<usize>(), r.next::<usize>());

    let mut pocketmons = Vec::<String>::with_capacity(n);

    let mut index_map = HashMap::<String, usize>::with_capacity(n);

    (0..n).for_each(|i|{
        let name = r.next::<String>();
        pocketmons.push(name.clone());
        index_map.insert(name, i);
    });

    (0..m).for_each(|_|{
        let name = r.next::<String>();
        if name.parse::<usize>().is_ok(){
            w.writeln(pocketmons[name.parse::<usize>().unwrap() - 1].clone());
        }else{
            w.writeln(index_map.get(&name).unwrap() + 1);
        }
    });
}