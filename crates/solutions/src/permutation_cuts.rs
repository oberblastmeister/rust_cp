use cp_library::prelude::*;

fn solve() {
    
}

fn run(_: usize, cin: &mut Cin, cout: &mut Cout) {
    
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
        assert_snapshot!(test_driver(run, TestKind::Many, "
"),
        @"
")
    }
}

