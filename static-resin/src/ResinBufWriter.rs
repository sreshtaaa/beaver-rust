use std::io::{BufWriter, Write, Error};
//use std::error;

use crate::policy;
use crate::filter;

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
    pub fn safe_write<A>(&mut self, buf: &policy::StringablePolicy<A>) -> Result<usize, Error> {
        match buf.export_check(&self.ctxt) {
            Ok(_) => {
                return self.buf_writer.write(buf.to_string().as_bytes());
            },
            Err(pe) => { // TODO: implement this so that it returns either an error::Error or an io::Error and not panic
                panic!("Policy check failed");
            },
        }
    }
}