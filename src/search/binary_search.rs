pub fn lower_bound_fn<T: Ord + PartialOrd>(func: impl Fn(i64) -> T, val: T, lo: i64, hi: i64) -> Result<i64, String> {
    let (mut low, mut high) = (lo, hi);
    if func(low) >= val || func(high) < val {
        return Err("Value out of bounds".to_string());
    }
    while low < high{
        let mid = (low + high) / 2;
        if func(mid) <= val {
            low = mid;
        } else {
            high = mid;
        }
    }
    Ok(low)
}

pub fn upper_bound_fn<T: Ord + PartialOrd>(func: impl Fn(i64) -> T, val: T, lo: i64, hi: i64) -> Result<i64, String> {
    let (mut low, mut high) = (lo, hi);
    if func(low) > val || func(high) <= val {
        return Err("Value out of bounds".to_string());
    }
    while low + 1 < high{
        let mid = (low + high) / 2;
        if func(mid) >= val {
            high = mid;
        } else {
            low = mid;
        }
    }
    Ok(high)
}