use std::io::{BufWriter, Write, Error};
//use std::error;

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
    -> Result<usize, Error> {
        match buf.get_policy().export_check(&self.ctxt) {
            Ok(_) => {
                return self.buf_writer.write(buf.to_string().as_bytes());
            },
            Err(pe) => { // TODO: implement this so that it returns either an error::Error or an io::Error and not panic
                panic!("Policy check failed");
            },
        }
    }
}

// problem: how to get information from StringablePolicy in a protected way?
// check rust access modifiers and what impact it has on the code. 
// factor out pieces from BEAVER library and things that come from app code.