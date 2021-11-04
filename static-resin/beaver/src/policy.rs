use std::fmt;
use crate::filter;
use std::error;

// ------------------- MAIN POLICY TRAITS/STRUCTS ----------------------------------
pub trait Policied<P : Policy> {
    fn get_policy(&self) -> Box<P>;
}

pub trait Policy {
    fn export_check(&self, ctxt: &filter::Context) -> Result<(), PolicyError>; 
    fn merge(self, _other: Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError>;
}

#[derive(Debug, Clone)]
pub struct PolicyError { pub message: String }

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self.message)
    }
}

impl error::Error for PolicyError {
    fn description(&self) -> &str {
        &self.message
    }
}

// ------------------- LIBRARY POLICY STRUCTS --------------------------------------
pub struct MergePolicy {
    policy1: Box<dyn Policy>,
    policy2: Box<dyn Policy>,
}

impl MergePolicy {
    pub fn make(policy1: Box<dyn Policy>, policy2: Box<dyn Policy>) -> MergePolicy {
        MergePolicy { policy1, policy2 }
    }
}

impl Policy for MergePolicy {
    fn export_check(&self, ctxt: &filter::Context) -> Result<(), PolicyError> {
        match *self.policy1.export_check(ctxt) {
            Ok(_) => {
                match *self.policy2.export_check(ctxt) {
                    Ok(_) => { Ok(()) },
                    Err(pe) => { Err(pe) }
                }
            },
            Err(pe) => { Err(pe) }
        }
    }

    fn merge(self, _other: Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError> {
        Ok(Box::new(MergePolicy { 
            policy1: Box::new(self),
            policy2: _other,
        }))
    }
}


pub struct PoliciedString<P : Policy> {
    pub(crate) string: String, 
    policy: P,
}

impl<P : Policy> PoliciedString<P> {
    pub fn make(string: String, policy: P) -> PoliciedString<P> {
        PoliciedString {
            string, policy
        }
    }

    pub fn push_str(&mut self, string: &str) -> PoliciedString<P> {
        PoliciedString {
            string: self.string.vec.extend_from_slice(string.as_bytes()),
            policy: self.policy,
        }
    }

    pub fn push_policy_str<O : Policy>(&mut self, policy_string: &PoliciedString<O>) 
    -> Result<PoliciedString<P>, policy::PolicyError> {
        match self.policy.merge(policy_string.policy) {
            Ok(p) => {
                Ok(PoliciedString {
                    string: self.string.vec.extend_from_slice(policy_string.string.as_bytes()),
                    policy: *p,
                })
            },
            Err(pe) => { Err(pe) }
        }
        
    }
} 

impl<P : Policy + Clone> Policied<P> for PoliciedString<P> {
    fn get_policy(&self) -> Box<P> { Box::new(self.policy.clone()) }
}

pub struct PoliciedNumber<P : Policy> {
    pub(crate) number: i64,  
    policy: P,
}

impl<P : Policy> PoliciedNumber<P> {
    pub fn make(number: i64, policy: P) -> PoliciedNumber<P> {
        PoliciedNumber {
            number, policy
        }
    }
} 

impl<P : Policy + Clone> Policied<P> for PoliciedNumber<P> {
    fn get_policy(&self) -> Box<P> { Box::new(self.policy.clone()) }
}
