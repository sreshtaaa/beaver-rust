use std::fmt;
use crate::filter;
use std::error::Error;

pub trait Policy<A> {
    fn export_check(&self, ctxt: &filter::Context) -> Result<(), PolicyError>; 
    fn merge(self, _other: Box<dyn Policy<A>>) -> Result<Box<dyn Policy<A>>, PolicyError>;
}

pub trait StringablePolicy<A> : Policy<A> {
    fn to_string(&self) -> String; // TODO: think about how to get data out such that only filter object can do so
                                   // one thought: force it to call export_check
}

#[derive(Debug, Clone)]
pub struct PolicyError {
    message: String,
}

// impl generic error trait for policerror
impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl Error for PolicyError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub struct Grade {
    student_id: String, 
    grade: i64, 
}

impl Grade {
    pub fn make(student_id: String, grade: i64) -> Grade {
        Grade {
            student_id, grade
        }
    }
}

impl Policy<Grade> for Grade {
    fn export_check(&self, ctxt: &filter::Context) -> Result<(), PolicyError> {
       match ctxt {
            filter::Context::File(fc) => {
                // pretend studentId is the filename
                if fc.file_name.eq(&self.student_id) {
                    return Ok(());
                } else {
                    return Err(PolicyError { message: "File must belong to same student".to_string() })
                }
            },
            filter::Context::ClientNetwork(_) => { 
               return Err(PolicyError { message: "Cannot send grade over network".to_string() });
            },
            filter::Context::ServerNetwork(_) => { 
                return Err(PolicyError { message: "Cannot send grade over network".to_string() });
            },
       }
    }
    fn merge(self, _other: Box<dyn Policy<Grade>>) ->  Result<Box<dyn Policy<Grade>>, PolicyError>{
        return Err(PolicyError { message: "Cannot merge grades".to_string() });
    }
}

impl StringablePolicy<Grade> for Grade {
    fn to_string(&self) -> String {
        return format!("{} {}", self.student_id, self.grade);
    }
}
