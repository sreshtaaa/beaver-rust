use std::error::Error;
use std::io;
use std::io::{BufWriter, Write, BufReader, Read};

use crate::policy;
use crate::filter;
use crate::policy::Policied;
use crate::policy::PolicyError;

extern crate serde; // Why do we have to use normal serde here but erased_serde in policy.rs? 

use std::net;

pub fn export(context: &filter::Context, s: &policy::PoliciedString) -> Result<String, Box<PolicyError>> {
    match s.get_policy().export_check(&context) {
        Ok(_) => { Ok(s.string.clone()) }, 
        Err(pe) => { Err(Box::new(pe)) }
    }
}

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

    pub fn safe_write_serialized(&mut self, buf: &policy::PoliciedString) -> Result<usize, Box<dyn Error>> {
        match buf.get_policy().export_check(&self.ctxt) {
            Ok(_) => {
                match self.buf_writer.write(format!("{}\n", buf.string).as_bytes()) {
                    Ok(s) => { Ok(s) }, 
                    Err(e) => { Err(Box::new(e)) }
                }
            }, 
            Err(pe) => { Err(Box::new(pe)) }
        }
    }

    pub fn safe_serialize_json<P: Policied + serde::Serialize>(&mut self, buf: &Box<P>)
    -> Result<usize, Box<dyn Error>> {
        match buf.get_policy().export_check(&self.ctxt) {
            Ok(_) => { // TODO: Abstract into serializer and bufwriter separately
                match self.buf_writer.write(format!("{}\n", serde_json::to_string(&*buf).unwrap()).as_bytes()) {
                    Ok(s) => { Ok(s) },
                    Err(e) => { Err(Box::new(e)) }
                }
            },
            Err(pe) => { 
                match &self.ctxt {
                    filter::Context::ClientNetwork(_) => {
                        self.buf_writer.write(format!("Beaver Error: {}\n", pe).as_bytes());
                        Err(Box::new(pe))
                    },
                    _ => Err(Box::new(pe)),
                }
            }
        }
    }
}

// pub struct BeaverBufReader<R: Read> {
//     buf_reader: BufReader<R>,
//     ctxt: filter::Context,
// }

// impl<R: Read> BeaverBufReader<R> {
//     pub fn safe_create(inner: R, context: filter::Context) -> BeaverBufReader<R> {
//         BeaverBufReader {
//             buf_reader: BufReader::new(inner), 
//             ctxt: context,
//         }
//     }

//     pub fn safe_read<P: Policied + serde::Deserialize>(&mut self, buf: &mut Box<P>) -> ? {
//         for line in self.buf_reader.lines() {

//         }
//     }
// }