use algorithm::io::{Reader, Writer};

fn solve(lis: &Vec<i32>, op: &mut Vec<usize>, idx: usize, curr: i32) -> (i32, i32) {
    if idx >= lis.len() {
        return (curr, curr);
    }

    let num = lis[idx];
    let mut max = i32::MIN;
    let mut min = i32::MAX;

    if op[0] > 0 {
        op[0] -= 1;
        let (res_max, res_min) = solve(lis, op, idx + 1, curr + num);
        max = max.max(res_max);
        min = min.min(res_min);
        op[0] += 1;
    }
    if op[1] > 0 {
        op[1] -= 1;
        let (res_max, res_min) = solve(lis, op, idx + 1, curr - num);
        max = max.max(res_max);
        min = min.min(res_min);
        op[1] += 1;
    }
    if op[2] > 0 {
        op[2] -= 1;
        let (res_max, res_min) = solve(lis, op, idx + 1, curr * num);
        max = max.max(res_max);
        min = min.min(res_min);
        op[2] += 1;
    }
    if op[3] > 0 {
        op[3] -= 1;
        let (res_max, res_min) = solve(lis, op, idx + 1, curr / num);
        max = max.max(res_max);
        min = min.min(res_min);
        op[3] += 1;
    }

    (max, min)
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    let lis = (0..n).map(|_| r.next::<i32>()).collect::<Vec<i32>>();
    let mut op = (0..4).map(|_| r.next::<usize>()).collect::<Vec<usize>>();

    let (max, min) = solve(&lis, &mut op, 1, lis[0]);

    w.writeln(max);
    w.writeln(min);
}
