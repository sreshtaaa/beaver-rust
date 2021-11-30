use std::error::Error;
use std::io;
use std::io::{BufWriter, Write, BufReader, Read, BufRead};

use crate::policy;
use crate::filter;
use crate::policy::Policied;

extern crate serde; // Why do we have to use normal serde here but erased_serde in policy.rs? 

use std::net;

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

    // TODO: Add other safe serialize methods (xml, other formats)
}

pub struct BeaverBufReader<R: Read> {
    buf_reader: BufReader<R>,
    ctxt: filter::Context,
}

impl<R: Read> BeaverBufReader<R> {
    pub fn safe_create(inner: R, context: filter::Context) -> BeaverBufReader<R> {
        BeaverBufReader {
            buf_reader: BufReader::new(inner), 
            ctxt: context,
        }
    }

    pub fn safe_read_line(&mut self) -> String {
        let mut deserialized_string = String::new(); 
        self.buf_reader.read_line(&mut deserialized_string);
        deserialized_string
    }
    /*
    pub fn safe_read<P: Policied + serde::Deserialize>(&mut self) -> ? {
        let mut deserialized_string = String::new();
        for line in self.buf_reader.lines() {
            
        }
    }
    */
}