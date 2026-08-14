use cp_library::prelude::*;

pub fn solve(n: usize, a: Vec<usize>, b: Vec<usize>) -> bool {
    let mut next: Vec<Option<usize>> = vec![None; n];
    let mut next_one: Vec<Option<usize>> = vec![None; n];
    if a[End] == 1 || b[End] == 1 {
        next_one[End] = Some(n - 1);
    }
    let mut next_for: Vec<Option<usize>> = vec![None; n + 2];
    next_for[a[End]] = Some(n - 1);
    next_for[b[End]] = Some(n - 1);
    for i in (0..(n - 1)).rev() {
        next_one[i] = next_one[i + 1];
        if a[End] == 1 || b[End] == 1 {
            next_one[i] = Some(i);
        }
        if a[i] == b[i] {
            next[i] = next_for[a[i] + 1];
        }
        next_for[a[i]] = Some(i);
        next_for[b[i]] = Some(i);
    }
    let mut dp: Vec<usize> = vec![0; n];
    for i in (0..n).rev() {
        if a[i] == b[i] {}
    }
    todo!();
}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {}
}
