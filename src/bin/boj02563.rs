use algorithm::math::{z_modulo_k, ZModuloK};
use algorithm::io::{Reader, Writer};
use algorithm::math::linalg::*;


fn main() {
    let mut r = Reader::new();
    let mut w = Writer::new();
    let mut _arr = Vec::new();
    _arr.resize_with(10000, i32::default);
    let mut papers = Matrix2::new(100, 100, _arr).unwrap();

    let mut paint = |i: usize, j: usize|{
        let end_i = (i + 10).min(100);
        let end_j = (j + 10).min(100);
        (i..end_i).for_each(
            |_i|{
                (j..end_j).for_each(|_j|{
                    papers[(_i, _j)] = papers[(_i, _j)] | 1;
                });
            }
        );
    };

    let mut tc = r.next::<usize>();
    while tc != 0{
        let (i, j) = (r.next::<usize>(), r.next::<usize>());
        paint(i, j);
        tc -= 1;
    }

    w.write(papers._arr.iter().sum::<i32>());
}