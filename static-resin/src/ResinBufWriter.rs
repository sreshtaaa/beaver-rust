use std::io;

mod filter;
pub use crate::filter;

pub struct ResinBufWriter<W: Write> {
    bufWriter: io::BufWriter<W: Write>,
    ctxt: filter::Context,
}

impl ResinBufWriter<W> {
    fn safe_write(&mut self, buf: &Policy<[u8]>) -> Result<usize> {
        match buf.export_check(self.ctxt) {
            Ok => {
                // perhaps serialize the policy
                return self.bufWriter.write(buf.get(...));
            },
            Err(pe: PolicyError) => {
                return Err(pe);
            },
        }
    }
}