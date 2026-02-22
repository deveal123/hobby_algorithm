use algorithm::io::{Reader, Writer};
use algorithm::string::*;
fn main(){
    let mut r = Reader::new();
    let mut w = Writer::new();

    let mut count = 0;
    while r.try_next::<String>().is_ok(){
        count += 1;
    }
    w.write(count);
}