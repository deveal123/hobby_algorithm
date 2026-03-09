use algorithm::io::{Reader, Writer};

#[derive(Clone)]
struct Mat {
    val: [usize; 4],
}

impl Mat {
    fn new() -> Self {
        Self { val: [1, 1, 1, 0] }
    }

    fn mul(&self, rhs: &Mat) -> Self {
        Mat {
            val: [
                ((self.val[0] * rhs.val[0]) % 1_000_000_007
                    + (self.val[1] * rhs.val[1]) % 1_000_000_007)
                    % 1_000_000_007,
                ((self.val[0] * rhs.val[2]) % 1_000_000_007
                    + (self.val[1] * rhs.val[3]) % 1_000_000_007)
                    % 1_000_000_007,
                ((self.val[2] * rhs.val[0]) % 1_000_000_007
                    + (self.val[3] * rhs.val[1]) % 1_000_000_007)
                    % 1_000_000_007,
                ((self.val[2] * rhs.val[2]) % 1_000_000_007
                    + (self.val[3] * rhs.val[3]) % 1_000_000_007)
                    % 1_000_000_007,
            ],
        }
    }

    fn pow(&self, n: usize) -> Self {
        let mut res = Mat { val: [1, 0, 0, 1] };
        let mut base = self.clone();
        let mut exp = n;
        while exp > 0 {
            if exp & 1 == 1 {
                res = res.mul(&base);
            }
            base = base.mul(&base);
            exp >>= 1;
        }
        res
    }

    fn element(&self) -> [usize; 4] {
        self.val.clone()
    }
}

fn main() {
    let (mut r, mut w) = (Reader::new(), Writer::new());
    let n = r.next::<usize>();
    w.write(format!("{:?}", Mat::new().pow(n).element()[1]));
}
