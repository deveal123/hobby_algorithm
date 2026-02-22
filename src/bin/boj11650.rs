use algorithm::io::{Reader, Writer};

fn main(){
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();

    let mut coords = (0..n).map(|_|{
        let x = r.next::<i32>();
        let y = r.next::<i32>();
        (x, y)
    }).collect::<Vec<(i32, i32)>>();

    coords.sort();

    for (x, y) in coords {
        w.writeln(format!("{} {}", x, y));
    }
}