use algorithm::io::{Reader, Writer};

#[inline]
fn solve(
    sudoku: &mut Vec<Vec<usize>>,
    row_hash: &mut Vec<usize>,
    col_hash: &mut Vec<usize>,
    block_hash: &mut Vec<usize>,
    empty: &mut Vec<(Vec<usize>, usize, usize)>,
) -> bool {
    if empty.is_empty() {
        return true;
    }
    let (possible, x, y) = empty.pop().unwrap();
    for i in possible.iter() {
        if (1 << *i) & (row_hash[x] | col_hash[y] | block_hash[(x / 3) * 3 + y / 3]) != 0 {
            continue;
        }
        sudoku[x][y] = *i;
        row_hash[x] |= 1 << *i;
        col_hash[y] |= 1 << *i;
        block_hash[(x / 3) * 3 + y / 3] |= 1 << *i;
        if solve(sudoku, row_hash, col_hash, block_hash, empty) {
            return true;
        }
        sudoku[x][y] = 0;
        row_hash[x] &= !(1 << *i);
        col_hash[y] &= !(1 << *i);
        block_hash[(x / 3) * 3 + y / 3] &= !(1 << *i);
    }
    empty.push((possible, x, y));
    false
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let mut sudoku = (0..9)
        .map(|_| (0..9).map(|_| r.next::<usize>()).collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    let mut row_hash = (0..9)
        .map(|i| {
            let mut h = 0;
            for j in 0..9 {
                h |= 1 << sudoku[i][j];
            }
            h
        })
        .collect::<Vec<usize>>();

    let mut col_hash = (0..9)
        .map(|j| {
            let mut h = 0;
            for i in 0..9 {
                h |= 1 << sudoku[i][j];
            }
            h
        })
        .collect::<Vec<usize>>();

    let mut block_hash = (0..9)
        .map(|i| {
            let mut h = 0;
            let (x, y) = (i / 3, i % 3);
            for i in 0..3 {
                for j in 0..3 {
                    h |= 1 << sudoku[x * 3 + i][y * 3 + j];
                }
            }
            h
        })
        .collect::<Vec<usize>>();

    let mut empty = vec![];
    for i in 0..9 {
        for j in 0..9 {
            let h = block_hash[(i / 3) * 3 + j / 3] | row_hash[i] | col_hash[j];
            if sudoku[i][j] == 0 {
                let cnt = (0..=9).filter(|&k| h & (1 << k) == 0).collect::<Vec<_>>();
                empty.push((cnt, i, j));
            }
        }
    }
    solve(
        &mut sudoku,
        &mut row_hash,
        &mut col_hash,
        &mut block_hash,
        &mut empty,
    );
    for i in 0..9 {
        for j in 0..9 {
            w.write(sudoku[i][j]);
        }
        w.writeln("");
    }
}
