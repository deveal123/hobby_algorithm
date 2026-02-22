use algorithm::io::{Reader, Writer};
use std::collections::HashSet;

fn main(){
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let s = r.next::<String>();

    // This could be solved using trie data structure.
    // But in this problem, input size is small enough to be solved using brute force.
    
    let mut set = HashSet::new();
    for i in 0..s.len(){
        for j in i..s.len(){
            set.insert(&s[i..j+1]);
        }
    }
    w.write(set.len());
}