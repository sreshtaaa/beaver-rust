use std::error::Error;
use std::io;
use std::io::{BufWriter, Write};

use crate::policy;
use crate::filter;
use crate::policy::Policied;

extern crate serde; // Why do we have to use normal serde here but erased_serde in policy.rs? 

pub struct BeaverBufWriter<W: Write> {
    buf_writer: BufWriter<W>,
    ctxt: filter::Context,
}

impl<W: Write> BeaverBufWriter<W> {
    pub fn safe_create(inner: W, context: filter::Context) -> BeaverBufWriter<W> {
        BeaverBufWriter {
            buf_writer: BufWriter::new(inner), 
            ctxt: context,
        }
    }
    
    pub fn safe_write<P: Policied + serde::Serialize>(&mut self, buf: &Box<P>)
    -> Result<usize, Box<dyn Error>> {
        match buf.get_policy().export_check(&self.ctxt) {
            Ok(_) => {
                match self.buf_writer.write(serde_json::to_string(&*buf).unwrap().as_bytes()) {
                    Ok(s) => { Ok(s) },
                    Err(e) => { Err(Box::new(e)) }
                }
            },
            Err(pe) => { Err(Box::new(pe)) },
        }
    }
}
