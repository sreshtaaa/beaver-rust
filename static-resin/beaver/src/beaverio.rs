use std::error::Error;
use std::io::{BufWriter, Write, BufReader, Read, BufRead};

use serde::de::DeserializeOwned;

use crate::policy;
use crate::filter;
use crate::policy::{Policied, PolicyError};

extern crate serde;

pub fn export_and_release(context: &filter::Context, s: &policy::PoliciedString) -> Result<String, Box<PolicyError>> {
    match s.export_check(&context) {
        Ok(str) => { Ok(str.clone()) }, 
        Err(pe) => { Err(Box::new(pe)) }
    }
}
// TODO: Add just an export_check funciton that takes in: PoliciedString, Context, and returns the raw string
// Rationale: We need to make Beaver be able to work with other libraries (such as lettre::Email)

// pub fn raw_check()

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
        match buf.export_check(&self.ctxt) {
            Ok(s) => {
                match self.buf_writer.write(format!("{}\n", s).as_bytes()) {
                    Ok(us) => { Ok(us) }, 
                    Err(e) => { Err(Box::new(e)) }
                }
            }, 
            Err(pe) => { Err(Box::new(pe)) }
        }
    }

    pub fn safe_write_json<T, P: Policied<T> + serde::Serialize>(&mut self, buf: &Box<P>)
    -> Result<usize, Box<dyn Error>> {
        match buf.get_policy().check(&self.ctxt) {
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
    // TODO: Writing context with data 
}

pub struct BeaverBufReader<R: Read> {
    buf_reader: BufReader<R>,
    // ctxt: filter::Context,
}

impl<R: Read> BeaverBufReader<R> {
    pub fn safe_create(inner: R /*context: filter::Context*/) -> BeaverBufReader<R> {
        BeaverBufReader {
            buf_reader: BufReader::new(inner), 
            // ctxt: context,
        }
    }

    pub fn safe_deserialize_line<T: DeserializeOwned>(&mut self) -> T {
        let mut deserialized_string = String::new(); 
        self.buf_reader.read_line(&mut deserialized_string).unwrap(); // TODO: handle this 
        serde_json::from_reader(deserialized_string.as_bytes()).expect("Unable to deserialize data")
    }

    /* pub fn safe_read_raw(&mut self) -> String {
        let mut deserialized_string = String::new().as_bytes(); 
        self.buf_reader.read(&mut deserialized_string);
        deserialized_string.to_owned()
    }
    */

    /*
    pub fn safe_read<P: Policied + serde::Deserialize>(&mut self) -> ? {
        let mut deserialized_string = String::new();
        for line in self.buf_reader.lines() {
            
        }
    }
    */

    // TODO: Implement with TypeTag
}