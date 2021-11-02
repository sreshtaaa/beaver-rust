use std::fmt;
use crate::filter;
use std::error;

pub trait Policied<P : Policy> {
    fn get_policy(&self) -> Box<P>;
}

pub trait Policy {
    fn export_check(&self, ctxt: &filter::Context) -> Result<(), PolicyError>; 
    fn merge(&self, _other: Box<dyn Policy>) -> Result<Box<dyn Policy>, PolicyError>;
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