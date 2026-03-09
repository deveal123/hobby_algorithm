use std::collections::{BTreeSet, HashMap};

use algorithm::io::{Reader, Writer};

struct Word {
    val: String,
    cnt: usize,
}

impl Word {
    fn new(val: String) -> Self {
        Word { val, cnt: 0 }
    }

    fn add(&mut self) {
        self.cnt += 1;
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for Word {}

impl PartialOrd for Word {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.cnt != other.cnt {
            other.cnt.partial_cmp(&self.cnt)
        } else if self.val.len() != other.val.len() {
            other.val.len().partial_cmp(&self.val.len())
        } else {
            self.val.partial_cmp(&other.val)
        }
    }
}

impl Ord for Word {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.cnt != other.cnt {
            other.cnt.cmp(&self.cnt)
        } else if self.val.len() != other.val.len() {
            other.val.len().cmp(&self.val.len())
        } else {
            self.val.cmp(&other.val)
        }
    }
}

struct WordSet {
    set: HashMap<String, Word>,
}

impl WordSet {
    fn new() -> Self {
        WordSet {
            set: HashMap::new(),
        }
    }

    fn add(&mut self, val: String) {
        let word = self
            .set
            .entry(val.clone())
            .or_insert_with(|| Word::new(val));
        word.add();
    }

    fn ordering(self) -> BTreeSet<Word> {
        self.set.into_values().collect()
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let (n, m) = (r.next::<usize>(), r.next::<usize>());
    let mut set = WordSet::new();
    for _ in 0..n {
        set.add(r.next::<String>());
    }
    let set = set
        .ordering()
        .iter()
        .filter_map(|word| {
            if word.val.len() >= m {
                Some(word.val.clone())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();
    for word in set.iter() {
        w.writeln(word);
    }
}
