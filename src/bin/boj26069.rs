use algorithm::io::{Reader, Writer};
use std::collections::{HashMap, HashSet};

struct Person {
    name: String,
    can_dance: bool,
}

impl Person {
    fn new(name: String) -> Self {
        if name == "ChongChong" {
            return Self {
                name,
                can_dance: true,
            };
        }
        Self {
            name,
            can_dance: false,
        }
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut mp = HashMap::<String, Person>::new();
    for _ in 0..n {
        let name1 = r.next::<String>();
        let name2 = r.next::<String>();

        mp.entry(name1.clone())
            .or_insert_with(|| Person::new(name1.clone()));
        mp.entry(name2.clone())
            .or_insert_with(|| Person::new(name2.clone()));

        let p1_can_dance = mp.get(&name1).unwrap().can_dance;
        let p2_can_dance = mp.get(&name2).unwrap().can_dance;

        if p1_can_dance || p2_can_dance {
            mp.get_mut(&name1).unwrap().can_dance = true;
            mp.get_mut(&name2).unwrap().can_dance = true;
        }
    }
    let mut cnt = 0;
    for person in mp.values() {
        if person.can_dance {
            cnt += 1;
        }
    }
    w.write(cnt);
}
