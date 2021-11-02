use std::error::Error;
use std::io;
use std::io::{BufWriter, Write};

use crate::policy;
use crate::filter;
use crate::policy::Policied;

pub struct ResinBufWriter<W: Write> {
    buf_writer: BufWriter<W>,
    ctxt: filter::Context,
}

impl<W: Write> ResinBufWriter<W> {
    pub fn safe_create(inner: W, context: filter::Context) -> ResinBufWriter<W> {
        ResinBufWriter {
            buf_writer: BufWriter::new(inner), 
            ctxt: context,
        }
    }
    pub fn safe_write<A: policy::Policy + Clone>(&mut self, buf: &policy::PoliciedString<A>) 
    -> Result<usize, Box<dyn Error>> {
        match buf.get_policy().export_check(&self.ctxt) {
            Ok(_) => {
                match self.buf_writer.write(buf.string.as_bytes()) {
                    Ok(s) => { Ok(s) },
                    Err(e) => { Err(Box::new(e)) }
                }
            },
            Err(pe) => { Err(Box::new(pe)) },
        }
    }
}
