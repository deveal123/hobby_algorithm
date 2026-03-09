use algorithm::io::{Reader, Writer};
use algorithm::set::Bitset;
use std::time::{Duration, Instant};

fn main() {
    #[cfg(feature = "local")]
    let st = Instant::now();
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let power = (0..(n * n)).map(|_| r.next::<i32>()).collect::<Vec<_>>();
    let mut bs = Bitset::new(16_000_000);
    bs.insert(0);

    let sums = (0..n)
        .map(|i| {
            let mut s = 0;
            for j in (0..n) {
                s += power[(n * i + j) as usize];
                s += power[(n * j + i) as usize];
            }
            s
        })
        .collect::<Vec<_>>();

    let offset: i32 = power.iter().sum();

    for s in sums {
        let shifted = (&bs) << (s as usize);
        bs |= shifted;
    }

    let m = bs
        .items()
        .iter()
        .map(|i| (*i as i32 - offset).abs())
        .min()
        .unwrap();
    w.write(m);

    #[cfg(feature = "local")]
    {
        let duration = st.elapsed();
        w.writeln(format!("duration: {:?}", duration));
    }
}
