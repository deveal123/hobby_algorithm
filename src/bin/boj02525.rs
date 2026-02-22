use algorithm::io::{Reader, Writer};
fn main(){
    let mut r = Reader::new();
    let mut w = Writer::new();

    let hour = r.next::<i32>();
    let minute = r.next::<i32>();
    let duration = r.next::<i32>();

    let time = (hour * 60 + minute + duration) % 1440;

    w.write(format_args!("{} {}", time / 60, time % 60));

}