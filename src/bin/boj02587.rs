use algorithm::io::{Reader, Writer};

fn main() {
    let (mut reader, mut writer) = (Reader::new(), Writer::new());

    let mut scores = (0..5).map(|_| reader.next::<i32>()).collect::<Vec<_>>();
    scores.sort();

    let mean = scores.iter().sum::<i32>() / 5;
    let median = scores[2];

    writer.writeln(mean);
    writer.writeln(median);
}
