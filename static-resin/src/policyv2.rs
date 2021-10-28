use std::fmt;
use crate::filter;

pub trait Policied<P : Policy> {
    fn get_policy(&self) -> Box<P>;
}

pub trait Policy {
    fn export_check(&self, ctxt: filter::Context) -> Result<(), PolicyError>; 
    fn merge(&self, _other: Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError>;
}

#[derive(Debug, Clone)]
pub struct PolicyError { message: String }

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

#[derive(Clone)]
pub struct GradePolicy { pub studentId: String }

impl Policy for GradePolicy {
    fn export_check(&self, ctxt: filter::Context) -> Result<(), PolicyError> {
        match ctxt {
             filter::Context::File(fc) => {
                 // pretend studentId is the filename
                 if fc.file_name.eq(&self.studentId) {
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
     fn merge(&self, _other: Box<dyn Policy>) ->  Result<Box<dyn Policy>, PolicyError>{
         return Err(PolicyError { message: "Cannot merge grades".to_string() });
     }
}

pub struct Grade {
    studentId: String, 
    grade: i64, 
    policy: GradePolicy,
}

// TODO: optimize clone() away
// can be hidden away
impl Policied<GradePolicy> for Grade {
    fn get_policy(&self) -> Box<GradePolicy> { 
        Box::new(self.policy.clone())
    }
}

pub struct PoliciedString<P : Policy> {
    string: String, 
    policy: P,
}

impl<P : Policy + Clone> Policied<P> for PoliciedString<P> {
    fn get_policy(&self) -> Box<P> { Box::new(self.policy.clone()) }
}

pub struct PoliciedNumber<P : Policy> {
    number: i64,  
    policy: P,
}

impl<P : Policy + Clone> Policied<P> for PoliciedNumber<P> {
    fn get_policy(&self) -> Box<P> { Box::new(self.policy.clone()) }
}

impl Grade {
    pub fn make(studentId: String, grade: i64, policy: GradePolicy) -> Grade {
        Grade {
            studentId, grade, policy
        }
    }

    pub fn get_studentId(&self) -> Box<PoliciedString<GradePolicy>> {
        return Box::new(PoliciedString {
            string: self.studentId.clone(),
            policy: self.policy.clone()
        });
    }

    pub fn get_grade(&self) -> Box<PoliciedNumber<GradePolicy>> {
        return Box::new(PoliciedNumber {
            number: self.grade,
            policy: self.policy.clone()
        });
    }
}
