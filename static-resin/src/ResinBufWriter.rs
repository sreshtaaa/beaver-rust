use std::io;

mod filter;
pub use crate::filter;

pub struct ResinBufWriter<W: Write> {
    bufWriter: io::BufWriter<W: Write>,
    ctxt: filter::Context,
}

impl ResinBufWriter<W> {
    fn safe_write(&mut self, buf: &StringablePolicy<Grade>) -> Result<usize> {
        match buf.export_check(self.ctxt) {
            Ok(_) => {
                return self.bufWriter.write(buf.toString());
            },
            Err(pe: PolicyError) => {
                return Err(pe);
            },
        }
    }
}