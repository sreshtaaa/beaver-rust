use beaver::{policy, filter};
use beaver::policy::{Policy, Policied, PolicyError, NonePolicy, PoliciedString, Policiedi64};
extern crate beaver_derive;
use beaver_derive::Policied;
use beaver::derive_policied;

#[derive(Clone, Serialize)]
pub struct GradePolicy { 
    pub student_id: String,
    pub instructor_id: String, 
    pub student_ip: Option<String>, 
    pub instructor_ip: Option<String>, 
}

impl Policy for GradePolicy {
    fn check(&self, ctxt: &filter::Context) -> Result<(), PolicyError> {
        match ctxt {
            filter::Context::File(fc) => {
                if fc.file_name.eq(&self.student_id) || fc.file_name.eq(&self.instructor_id) {
                    return Ok(());
                } else {
                    return Err(PolicyError { message: "File must belong to same student".to_string() })
                }
                /*
                match fc.permission {
                    filter::Permission::ReadOnly => Err(PolicyError { message: "Cannot write data with ReadOnly policy".to_string() }), 
                    _ =>  {  // pretend student_id is the filename
                        if fc.file_name.eq(&self.student_id) || fc.file_name.eq(&self.instructor_id) {
                            return Ok(());
                        } else {
                            return Err(PolicyError { message: "File must belong to same student".to_string() })
                        }
                    }, 
                }
                */
            },
            filter::Context::ClientNetwork(rcc) => { 
                //let self_ip = "127.0.0.1".to_string();

                // Sample filter: only send data to trusted ip addresses
                if opt_eq(&rcc.remote_ip_address.to_string(), &self.student_ip) || 
                    opt_eq(&rcc.remote_ip_address.to_string(), &self.instructor_ip) 
                    //|| rcc.remote_ip_address.to_string().eq(&self_ip)
                {
                    return Ok(()); 
                } else {
                    return Err(PolicyError { message: "Cannot send data to untrusted IP Address".to_string() })
                }

            },
            filter::Context::ServerNetwork(_) => { 
                return Err(PolicyError { message: "Cannot send grade over network".to_string() });
            },
            filter::Context::CustomContext(_) => { 
                return Err(PolicyError { message: "Custom contexts not enabled for grades".to_string() });
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

fn opt_eq<T: std::cmp::PartialEq>(obj1: &T, obj2: &Option<T>) -> bool {
    match obj2.as_ref() {
        None => false, 
        Some(v) => obj1 == v,
    }
}

//#[derive(Policied, Serialize)]
// #[policied(PoliciedGrade)]
#[derive(Serialize, Clone)]
pub struct Grade {
    //#[policy_protected(PoliciedString)] 
    pub student_id: String, 

    //#[policy_protected(PoliciedNumber)] 
    pub grade: i64, 
}

derive_policied!(Grade, PoliciedGrade);

impl PoliciedGrade {
    pub fn make_decomposed_unpolicied(student_id: String, grade: i64, policy: Box<dyn Policy>) -> PoliciedGrade {
        PoliciedGrade { 
            inner: Grade {
                student_id, grade
            },
            policy
        }
    }
    pub fn make_decomposed_policied(student_id: PoliciedString, grade: Policiedi64, policy: Box<dyn Policy>) -> PoliciedGrade {
        PoliciedGrade { 
            inner: Grade {
                student_id: student_id.export(), 
                grade: grade.export()
            },
            policy: student_id.get_policy().merge(grade.get_policy()).unwrap()
        }
    }
    pub fn student_id(&self) -> PoliciedString {
        PoliciedString::make(self.inner.clone().student_id, self.policy.clone())
    }
    pub fn grade(&self) -> Policiedi64 {
        Policiedi64::make(self.inner.clone().grade, self.policy.clone())
    }
}
