use std::{io, fmt};

pub struct IoWriteAsFmtWrite<W: io::Write>(W);

impl<W: io::Write> IoWriteAsFmtWrite<W> {
    pub fn new(w: W) -> Self {
        IoWriteAsFmtWrite(w)
    }
}

impl<W: io::Write> fmt::Write for IoWriteAsFmtWrite<W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.0.write(s.as_bytes()).unwrap();
        Ok(())
    }
}
