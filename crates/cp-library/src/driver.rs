pub use crate::cio::{Cin, Cout};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TestKind {
    Single,
    Many,
}

pub fn driver_with_io<F>(
    mut solve: F,
    test_kind: TestKind,
    reader: impl std::io::Read + 'static,
    writer: impl std::io::Write + 'static,
) where
    F: FnMut(usize, &mut Cin, &mut Cout),
{
    let mut cin = Cin::from_reader(reader);
    let mut cout = Cout::from_write(writer);
    let test_count = match test_kind {
        TestKind::Single => 1,
        TestKind::Many => cin.read(),
    };

    for t in 0..test_count {
        solve(t, &mut cin, &mut cout);
    }

    cout.flush();
}

pub fn driver<F>(solve: F, test_kind: TestKind)
where
    F: FnMut(usize, &mut Cin, &mut Cout),
{
    driver_with_io(solve, test_kind, std::io::stdin(), std::io::stdout());
}

pub fn test_driver<F>(solve: F, test_kind: TestKind, input: &str) -> String
where
    F: FnMut(usize, &mut Cin, &mut Cout),
{
    use std::cell::RefCell;
    use std::io::{self, Write};
    use std::rc::Rc;

    struct SharedWriter(Rc<RefCell<Vec<u8>>>);

    impl Write for SharedWriter {
        fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
            self.0.borrow_mut().extend_from_slice(buffer);
            Ok(buffer.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    let output = Rc::new(RefCell::new(Vec::new()));
    driver_with_io(
        solve,
        test_kind,
        std::io::Cursor::new(input.as_bytes().to_vec()),
        SharedWriter(Rc::clone(&output)),
    );

    let output = output.borrow().clone();
    String::from_utf8(output).expect("driver output was not valid UTF-8")
}
