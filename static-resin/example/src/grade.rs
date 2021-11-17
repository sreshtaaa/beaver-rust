use beaver::{policy, filter};
use beaver::policy::{Policy, Policied, PolicyError};
extern crate beaver_derive;
use beaver_derive::Policied;

#[derive(Clone)]
pub struct GradePolicy { 
    pub student_id: String,
    pub instructor_id: String
}

impl Policy for GradePolicy {
    fn export_check(&self, ctxt: &filter::Context) -> Result<(), PolicyError> {
        match ctxt {
             filter::Context::File(fc) => {
                 // pretend student_id is the filename
                 if fc.file_name.eq(&self.student_id) || fc.file_name.eq(&self.instructor_id) {
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
     fn merge(&self, other: &Box<dyn Policy>) ->  Result<Box<dyn Policy>, PolicyError>{
        Ok(Box::new(policy::MergePolicy::make( 
            Box::new(self.clone()),
            other.clone(),
        )))
     }
}

#[derive(Policied)]
pub struct Grade {
    #[policy_protected] student_id: String, 
    #[policy_protected] grade: i64, 
    policy: Box<dyn Policy>,
}

impl Grade {
    pub fn make(student_id: String, grade: i64, policy: Box<GradePolicy>) -> Grade {
        Grade {
            student_id, grade, policy
        }
    }

    // can be hidden away
    pub fn get_student_id(&self) -> policy::PoliciedString {
        return policy::PoliciedString::make(
            self.student_id.clone(),
            self.policy.clone()
        );
    }

    // can be hidden away
    pub fn get_grade(&self) -> policy::PoliciedNumber {
        return policy::PoliciedNumber::make(
            self.grade,
            self.policy.clone()
        );
    }
}
