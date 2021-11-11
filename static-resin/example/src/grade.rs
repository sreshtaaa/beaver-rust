use beaver::{policy, filter};
use beaver::policy::Policy;

#[derive(Clone)]
pub struct GradePolicy { 
    pub student_id: String,
    pub instructor_id: String
}

impl policy::Policy for GradePolicy {
    fn export_check(&self, ctxt: &filter::Context) -> Result<(), policy::PolicyError> {
        match ctxt {
             filter::Context::File(fc) => {
                 // pretend student_id is the filename
                 if fc.file_name.eq(&self.student_id) || fc.file_name.eq(&self.instructor_id) {
                     return Ok(());
                 } else {
                     return Err(policy::PolicyError { message: "File must belong to same student".to_string() })
                 }
             },
             filter::Context::ClientNetwork(_) => { 
                return Err(policy::PolicyError { message: "Cannot send grade over network".to_string() });
             },
             filter::Context::ServerNetwork(_) => { 
                 return Err(policy::PolicyError { message: "Cannot send grade over network".to_string() });
             },
        }
     }
     fn merge(&self, other: &Box<dyn Policy>) ->  Result<Box<dyn policy::Policy>, policy::PolicyError>{
        Ok(Box::new(policy::MergePolicy::make( 
            Box::new(self.clone()),
            other.clone(),
        )))
     }
}

pub struct Grade {
    student_id: String, 
    grade: i64, 
    policy: Box<dyn Policy>,
}

// TODO: optimize clone() away
// can be hidden away
impl policy::Policied for Grade {
    fn get_policy(&self) -> &Box<dyn Policy> { 
        &self.policy
    }
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
