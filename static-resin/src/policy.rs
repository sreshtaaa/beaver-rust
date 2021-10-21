use std::fmt;

use crate::filter;

trait Policy<A> {
    fn export_check(self, ctxt: filter::Context) -> Result<(), PolicyError>; 
    fn merge(self, _other: Policy<A>) -> Result<Policy<A>, PolicyError>;
}

struct Grade {
    studentId: String, 
    grade: i32, 
}

impl Policy<Grade> for Grade {
    fn export_check(self, ctxt: filter::Context) -> Result<(), PolicyError> {
       match ctxt {
            filter::Context::File(fc) => {
                // pretend studentId is the filename
                if (fc.file_name.eq(self.studentId)) {
                    return Ok(());
                } else {
                    return Err(PolicyError { message: "File must belong to same student" })
                }
            },
            filter::Context::ClientNetwork => { 
               return Err(PolicyError { message: "Cannot send grade over network" });
            },
            filter::Context::ServerNetwork => { 
                return Err(PolicyError { message: "Cannot send grade over network" });
            },
       }
    }
    fn merge(self, _other: Policy<Grade>) -> Result<Policy<Grade>, PolicyError>{
        return Err(PolicyError { message: "Cannot merge grades"});
    }
}

#[derive(Debug, Clone)]
struct PolicyError {
    message: String,
}

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message);
    }
}
