use cp_library::{
    Cin, Cout, End, Itertools, PrefixSum, algebra::AddMonoid, virtual_partition_point,
};

#[derive(Debug)]
struct Info {
    heights: Vec<usize>,
    heights_sum: PrefixSum<AddMonoid<isize>>,
}

impl Info {
    fn new(i: usize, heights: &[usize]) -> Info {
        let mut heights = heights.to_vec();
        for j in (0..i).rev() {
            heights[j] = heights[j].max(heights[j + 1]);
        }
        for j in (i + 1)..heights.len() {
            heights[j] = heights[j].max(heights[j - 1]);
        }
        let heights_sum = heights.clone().into_iter().map(|x| x as isize).collect();
        Info {
            heights,
            heights_sum,
        }
    }
}

fn solve(grid_height: usize, heights: Vec<usize>) -> usize {
    if heights.len() == 1 {
        return grid_height - heights[0];
    }
    let n = heights.len();
    let infos = (0..n).map(|i| Info::new(i, &heights)).collect_vec();
    let mut res = n * grid_height;
    for i in 0..n {
        for j in (i + 1)..n {
            let pi = virtual_partition_point(i, j, |k| infos[i].heights[k] <= infos[j].heights[k]);
            let new_res = if pi == i {
                infos[j].heights_sum.query(..) as usize
            } else {
                let r1 = infos[i].heights_sum.query(..i) as usize;
                let r2 = infos[i].heights_sum.query(i..pi) as usize;
                let r3 = infos[j].heights_sum.query(pi..j) as usize;
                let r4 = infos[j].heights_sum.query(j..) as usize;
                r1 + r2 + r3 + r4
            };
            res = res.min(new_res);
        }
    }
    n * grid_height - res
}

fn main() {
    let mut cin = Cin::new();
    let mut cout = Cout::new();
    let t: usize = cin.read();
    for _ in 0..t {
        let n: usize = cin.read();
        let h: usize = cin.read();
        let a: Vec<usize> = cin.read_vec(n);
        let res = solve(h, a);
        cout.println(res);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(solve(4, vec![1, 3, 1, 2, 3, 1, 1]), 14);
        assert_eq!(solve(10, vec![7, 5, 1, 3, 2, 5, 6, 8]), 43);
        assert_eq!(solve(1, vec![1]), 0);
        assert_eq!(solve(20, vec![5, 2, 1, 2, 1, 3, 6, 7, 1, 1]), 170);
        assert_eq!(
            solve(1000000000, vec![1, 420420420, 1, 420420420, 1]),
            3738738738
        );
        assert_eq!(solve(1000000000, vec![1]), 999999999)
    }
}
