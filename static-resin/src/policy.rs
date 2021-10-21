use std::fmt;
use crate::filter;

trait Policy<A> {
    fn export_check(self, ctxt: filter::Context) -> Result<A, PolicyError>; 
    fn merge(self, _other: Box<dyn Policy<A>>) -> Result<Box<dyn Policy<A>>, PolicyError>;
}

trait StringablePolicy<A> : Policy<A> {
    fn toString(self) -> String;
}

pub struct Grade {
    studentId: String, 
    grade: i64, 
}

impl Grade {
    pub fn make(studentId: String, grade: i64) -> Grade {
        Grade {
            studentId, grade
        }
    }
}

impl Policy<Grade> for Grade {
    fn export_check(self, ctxt: filter::Context) -> Result<Grade, PolicyError> {
       match ctxt {
            filter::Context::File(fc) => {
                // pretend studentId is the filename
                if (fc.file_name.eq(&self.studentId)) {
                    return Ok(self);
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
    fn toString(self) -> String {
        return format!("{} {}", self.studentId, self.grade);
    }
}

#[derive(Debug, Clone)]
pub struct PolicyError {
    message: String,
}

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}
