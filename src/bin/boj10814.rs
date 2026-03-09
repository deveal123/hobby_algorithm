use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();

    let mut people = (0..n)
        .map(|i| {
            let age = r.next::<usize>();
            let name = r.next::<String>();

            (age, i, name)
        })
        .collect::<Vec<_>>();

    people.sort();

    people.iter().for_each(|(age, _, name)| {
        w.writeln(format!("{} {}", age, name));
    });
}
