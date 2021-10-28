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

// problem: how to get information from StringablePolicy in a protected way?
// check rust access modifiers and what impact it has on the code. 
// factor out pieces from BEAVER library and things that come from app code.