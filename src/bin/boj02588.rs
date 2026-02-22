use algorithm::io::{Reader, Writer};

fn main(){
    let mut r = Reader::new();
    let mut w = Writer::new();

    let num1 = r.next::<i32>();
    let num2 = r.next::<i32>();

    let (a, b, c) = (num2 / 100, (num2 % 100) / 10, num2 % 10);
    w.writeln(num1 * c);
    w.writeln(num1 * b);
    w.writeln(num1 * a);
    w.writeln(num1 * num2);

}