use algorithm::io::{Reader, Writer};

fn main(){
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();

    let mut numbers = (0..n).map(|i|{
        (r.next::<i32>(), i)
    }).collect::<Vec<_>>();

    numbers.sort_unstable();

    let mut rank = 0;
    let mut num = numbers[0].0;

    let mut ranks = vec![0; n];

    numbers.iter().for_each(|(n, i)|{
        if *n > num{
            rank += 1;
            num = *n;
        }
        ranks[*i] = rank;
    });

    ranks.iter().for_each(|r|{
        w.write(*r);
    });
}