use algorithm::io::{Reader, Writer};

fn score(chess: &Vec<Vec<u8>>, st_color: u8, other_color: u8, start_row: usize, start_col: usize) -> usize{
    let mut cnt = 0;
    for i in (start_row..start_row + 8){
        for j in (start_col..start_col + 8){
            let expected_color = if (i + j) % 2 == 0 { st_color } else { other_color };
            if chess[i][j] != expected_color {
                cnt += 1;
            }
        }
    }
    64 - cnt
}

fn main(){
    let (mut reader, mut writer) = (Reader::new(), Writer::new());
    let (m, n) = (reader.next::<usize>(), reader.next::<usize>());

    let chess = (0..m).map(|_|{
        let chess_row: Vec<u8> = reader.next::<String>().into_bytes();
        chess_row
    }).collect::<Vec<Vec<u8>>>();


    let mut max_score = 64;

    for i in 0..=m - 8{
        for j in 0..=n - 8{
            let score_beg_b = score(&chess, b'B', b'W', i, j);
            let score_beg_w = score(&chess, b'W', b'B', i, j);
            max_score = max_score.min(score_beg_b.min(score_beg_w));
        }
    }

    writer.writeln(max_score);
}