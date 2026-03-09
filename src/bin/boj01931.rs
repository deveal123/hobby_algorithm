use algorithm::io::{Reader, Writer};

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let mut meetings = (0..n)
        .map(|_| (r.next::<usize>(), r.next::<usize>()))
        .collect::<Vec<_>>();
    meetings.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    let mut count = 0;
    let mut last_end = 0;
    meetings.iter().for_each(|(start, end)| {
        if *start >= last_end {
            count += 1;
            last_end = *end;
        }
    });
    w.write(count);
}
