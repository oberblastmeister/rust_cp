#![no_main]

use arbitrary::Unstructured;
use cp_library::prelude::*;
use libfuzzer_sys::fuzz_target;

fn fuzz(mut un: arbitrary::Unstructured) -> arbitrary::Result<()> {
    let a: Vec<usize> = un.arbitrary()?;
    let b: Vec<usize> = un.arbitrary()?;
    if a.len() != b.len() {
        return Err(arbitrary::Error::IncorrectFormat);
    }
    let n = a.len();
    if !(a.iter().all(|&x| 1 <= x && x <= n) && b.iter().all(|&x| 1 <= x && x <= n)) {
        return Err(arbitrary::Error::IncorrectFormat);
    }
    solutions::good_schedule::solve(n, a, b);
    Ok(())
}

fuzz_target!(|data: &[u8]| {
    let unstructured = Unstructured::new(data);
    let _ = fuzz(unstructured);
});
