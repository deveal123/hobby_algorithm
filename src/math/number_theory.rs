pub struct Sieve{
    num_vec: Vec<bool>,
}

impl Sieve{
    pub fn new(n: usize) -> Self{
        let mut num_vec = Vec::new();
        num_vec.resize_with(((n >> 1) + 1), || true);
        num_vec[0] = false;

        for i in 1..(n >> 1 + 1){
            if !num_vec[i]{continue;}
            let mut j = 2 * i * (i + 1);
            while j < (n >> 1 + 1){
                num_vec[j] = false;
                j += ((i << 1) | 1);
            }
        }
        Self{
            num_vec,
        }
    }

    pub fn is_prime(&self, n: usize) -> Result<bool, String>{
        if (n > (self.num_vec.len() << 1) | 1){
            return Err(format!("Sieve : Sieve size is too small. n = {}, capacity = {}", n, self.num_vec.len() << 1 | 1))
        }

        if (n == 2) {
            Ok(true)
        } else if (n & 1 == 0) {
            Ok(false)
        } else {
            Ok(self.num_vec[n >> 1])
        }
    }

    pub fn sieve(&self) -> Vec<bool>{
        self.num_vec.clone()
    }
}