use cp_library::{multiset::MultiSet, prelude::*};

fn solve(actions: Vec<usize>, num_enemies: usize, num_secs: usize) -> usize {
    let mut enemies: MultiSet<usize> =
        iter::repeat_n(0, num_enemies.min(actions.len() + 1)).collect();
    let mut time = 0;
    let mut stack = actions.clone();
    stack.reverse();
    loop {
        time += 1;
        let enemy = enemies.pop_first().unwrap();
        enemies.insert(enemy + 1);
        if let Some(&action) = stack.last()
            && time == action
        {
            if enemies.len() > stack.len() {
                enemies.pop_last().unwrap();
            } else {
                enemies.pop_last().unwrap();
                enemies.insert(0);
            }
            stack.pop().unwrap();
        }
        if time == num_secs {
            return enemies.last().copied().unwrap();
        }
    }
}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    let n = cin.read();
    let m = cin.read();
    let l = cin.read();
    let a = cin.read_vec(n);
    let res = solve(a, m, l);
    cout.println(res);
}

pub fn main() {
    driver(run, TestKind::Many);
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn smoke() {
        assert_eq!(solve(vec![1, 4, 9, 16, 25], 1, 32), 7);
        assert_eq!(solve(vec![13, 37], 3, 40), 19);
        assert_snapshot!(test_driver(run, TestKind::Many, "
                    7
                    1 2 10
                    10
                    5 1 32
                    1 4 9 16 25
                    2 3 40
                    13 37
                    2 2 7
                    6 7
                    8 5 60
                    3 17 20 28 36 44 45 50
                    6 7 1987
                    6 7 66 77 666 777
                    1 1 1
                    1
        "), @"
        5
        7
        19
        1
        19
        1477
        0
        ");
    }
}
