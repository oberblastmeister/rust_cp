#![no_main]

use arbitrary::Unstructured;
use libfuzzer_sys::fuzz_target;
use cp_library::Itertools;

fuzz_target!(|data: &[u8]| {
    let mut unstructured = Unstructured::new(data);
    let Ok((a, mut ds)) = unstructured.arbitrary::<(usize, Vec<usize>)>() else {
        return;
    };
    if a > 1000 as usize {
        return;
    }
    if ds.is_empty() {
        return;
    }
    if !ds.iter().copied().all(|d| d <= 9) {
        return;
    }
    ds.sort();
    let ds: Vec<usize> = ds.into_iter().unique().collect();
    let res1 = solutions::cirno_and_the_number_easy::solve(a, &ds);
    let res2 = solutions::cirno_and_the_number_easy::brute(a, &ds);
    assert_eq!(res1, res2);
});
