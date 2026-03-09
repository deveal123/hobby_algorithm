use std::collections::BTreeSet;

use algorithm::io::{Reader, Writer};

fn vec_union<T: Ord + Copy + Clone>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut v = Vec::with_capacity(a.len() + b.len());
    let (mut iter_a, mut iter_b) = (a.iter(), b.iter());
    let (mut next_a, mut next_b) = (iter_a.next(), iter_b.next());
    while next_a.is_some() || next_b.is_some() {
        if next_a.is_none() {
            v.push(*next_b.unwrap());
            next_b = iter_b.next();
        } else if next_b.is_none() {
            v.push(*next_a.unwrap());
            next_a = iter_a.next();
        } else {
            let aa = next_a.unwrap();
            let bb = next_b.unwrap();
            if aa < bb {
                v.push(*aa);
                next_a = iter_a.next();
            } else if aa > bb {
                v.push(*bb);
                next_b = iter_b.next();
            } else {
                v.push(*aa);
                next_a = iter_a.next();
                next_b = iter_b.next();
            }
        }
    }
    v
}

fn generate_apocalypse_number(apocalypse_numbers: &Vec<usize>, step: usize) -> Vec<usize> {
    let big_step = 10usize.pow((step - 3) as u32);
    let mut new_apocalypse_numbers1: Vec<usize> = Vec::with_capacity(apocalypse_numbers.len() * 10);
    let mut new_apocalypse_numbers2: Vec<usize> = Vec::with_capacity(9 * big_step);
    for &number in apocalypse_numbers.iter() {
        for i in 0..10 {
            new_apocalypse_numbers1.push(10 * number + i);
        }
    }

    for i in big_step..big_step * 10 {
        new_apocalypse_numbers2.push(666 + i * 1000);
    }
    vec_union(&new_apocalypse_numbers1, &new_apocalypse_numbers2)
}

fn main() {
    let (mut reader, mut writer) = (Reader::new(), Writer::new());

    let mut apocalypse_numbers = vec![666usize];
    let apo2 = generate_apocalypse_number(&apocalypse_numbers, 3);
    let apo3 = generate_apocalypse_number(&apo2, 4);
    let apo4 = generate_apocalypse_number(&apo3, 5);
    let apo5 = generate_apocalypse_number(&apo4, 6);
    apocalypse_numbers.extend(apo2);
    apocalypse_numbers.extend(apo3);
    apocalypse_numbers.extend(apo4);
    apocalypse_numbers.extend(apo5);

    let n = reader.next::<usize>();
    writer.write(apocalypse_numbers[n - 1]);
}
