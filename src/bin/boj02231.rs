use algorithm::io::{Reader, Writer};

fn flatten_vec<T: Clone>(vec: &Vec<Vec<T>>) -> Vec<T> {
    let total_len: usize = vec.iter().map(|v| v.len()).sum();
    let mut flat_vec: Vec<T> = Vec::with_capacity(total_len);
    for v in vec.iter() {
        flat_vec.extend(v.iter().cloned());
    }
    flat_vec
}

fn get_digit_sum(n: isize) -> isize {
    let mut sum = 0;
    let mut num = n;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}

fn main() {
    let (mut reader, mut writer) = (Reader::new(), Writer::new());
    let n = reader.next::<isize>();
    let mut result = 0;
    let n_min = (n - 54).max(1);
    for i in n_min..n {
        if i + get_digit_sum(i) == n {
            result = i;
            break;
        }
    }
    writer.writeln(result);
}
