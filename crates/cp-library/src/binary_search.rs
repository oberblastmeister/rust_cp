// Works exactly like partition_point in rust std but operates in a "virtual" array
pub fn virtual_partition_point<F>(start: usize, end: usize, mut f: F) -> usize
where
    F: FnMut(usize) -> bool,
{
    assert!(start <= end);
    let mut lo = start;
    let mut hi = end;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if f(mid) {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}